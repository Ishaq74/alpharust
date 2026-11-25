//! Interface terminal (ratatui) : menu, lint, génération de projets, listes.
//!
//! L'état ([`App`]) est indépendant du terminal : les tests pilotent `handle_key`
//! et rendent dans un `TestBackend`.

use crate::engine::{self, composer};
use crate::parser::Rule;
use crate::profiles::Profile;
use crate::scaffold::{self, Event as ScEvent, Framework, Request};
use anyhow::{bail, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::{self, IsTerminal};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Lint,
    Generate(Framework),
    Doctor,
    Projects,
    ListRules,
    ListProfiles,
    ListCheckers,
    Quit,
}

pub const MENU: [(&str, Action); 11] = [
    ("Lint du répertoire courant", Action::Lint),
    (
        "Générer un projet Astro",
        Action::Generate(Framework::Astro),
    ),
    (
        "Générer un projet Laravel",
        Action::Generate(Framework::Laravel),
    ),
    (
        "Générer un projet Wagtail",
        Action::Generate(Framework::Wagtail),
    ),
    ("Générer un projet Rust", Action::Generate(Framework::Rust)),
    ("Projets générés (état, tâches)", Action::Projects),
    ("Vérifier les outils (doctor)", Action::Doctor),
    ("Lister les règles", Action::ListRules),
    ("Lister les profils", Action::ListProfiles),
    ("Lister les checkers", Action::ListCheckers),
    ("Quitter", Action::Quit),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Normal,
    Ok,
    Error,
    Warning,
    Info,
    Title,
}

#[derive(Debug, Clone)]
pub struct OutLine {
    pub text: String,
    pub level: Level,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Menu,
    /// Saisie du nom d'un projet à générer.
    Input {
        kind: Framework,
        buffer: String,
    },
}

pub struct App {
    pub selected: usize,
    pub mode: Mode,
    pub output: Vec<OutLine>,
    pub scroll: u16,
    pub status: String,
    pub profiles: Vec<String>,
    /// Index du profil actif (`None` = toutes les règles).
    pub profile: Option<usize>,
    pub quit: bool,
    /// Génération en cours : réception des événements du thread de travail.
    pub job: Option<mpsc::Receiver<ScEvent>>,
    pub cancel: Arc<AtomicBool>,
    /// Le résultat suit la fin du journal (génération en direct).
    pub follow: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

fn line(text: impl Into<String>, level: Level) -> OutLine {
    OutLine {
        text: text.into(),
        level,
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            selected: 0,
            mode: Mode::Menu,
            output: Vec::new(),
            scroll: 0,
            status: "Prêt".into(),
            profiles: Profile::list().unwrap_or_default(),
            profile: None,
            quit: false,
            job: None,
            cancel: Arc::new(AtomicBool::new(false)),
            follow: false,
        };
        app.output
            .push(line("Bienvenue dans AlphaRust", Level::Title));
        app.output.push(line(
            "Choisissez une action dans le menu (Entrée).",
            Level::Normal,
        ));
        app.output.push(line(
            "p : changer de profil · PgUp/PgDn : défiler · q : quitter",
            Level::Info,
        ));
        app
    }

    pub fn profile_name(&self) -> Option<&str> {
        self.profile
            .and_then(|i| self.profiles.get(i))
            .map(String::as_str)
    }

    fn set_output(&mut self, lines: Vec<OutLine>) {
        self.output = lines;
        self.scroll = 0;
        self.follow = false;
    }

    pub fn running(&self) -> bool {
        self.job.is_some()
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Release {
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.quit = true;
            return;
        }
        if let Mode::Input { kind, buffer } = &mut self.mode {
            match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Menu;
                    self.status = "Génération annulée".into();
                }
                KeyCode::Backspace => {
                    buffer.pop();
                }
                KeyCode::Char(c) if buffer.len() < 64 => buffer.push(c),
                KeyCode::Enter => {
                    let (kind, name) = (*kind, buffer.trim().to_string());
                    self.mode = Mode::Menu;
                    self.generate(kind, &name);
                }
                _ => {}
            }
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Up | KeyCode::Char('k') => self.selected = self.selected.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected + 1 < MENU.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Char('x') if self.running() => {
                self.cancel.store(true, Ordering::Relaxed);
                self.status = "Annulation demandée…".into();
            }
            KeyCode::PageDown => {
                self.follow = false;
                self.scroll = self.scroll.saturating_add(8);
            }
            KeyCode::PageUp => {
                self.follow = false;
                self.scroll = self.scroll.saturating_sub(8);
            }
            KeyCode::Char('p') => self.cycle_profile(),
            KeyCode::Enter => self.activate(MENU[self.selected].1),
            _ => {}
        }
    }

    fn cycle_profile(&mut self) {
        self.profile = match self.profile {
            None if !self.profiles.is_empty() => Some(0),
            Some(i) if i + 1 < self.profiles.len() => Some(i + 1),
            _ => None,
        };
        self.status = format!(
            "Profil : {}",
            self.profile_name().unwrap_or("toutes les règles")
        );
    }

    pub fn activate(&mut self, action: Action) {
        match action {
            Action::Quit => self.quit = true,
            Action::Lint => self.lint(),
            Action::Generate(kind) => {
                if self.running() {
                    self.status = "Une génération est déjà en cours (x : annuler)".into();
                    return;
                }
                self.mode = Mode::Input {
                    kind,
                    buffer: format!("mon-{}", kind.id()),
                };
                self.status = "Nom du projet (Entrée : valider, Échap : annuler)".into();
            }
            Action::Doctor => self.doctor(),
            Action::Projects => self.projects(),
            Action::ListRules => self.list_rules(),
            Action::ListProfiles => self.list_profiles(),
            Action::ListCheckers => {
                let mut out = vec![line("Checkers sémantiques", Level::Title)];
                for (n, d) in crate::checkers::catalog() {
                    out.push(line(format!("{n:<20} {d}"), Level::Normal));
                }
                self.status = format!("{} checkers", out.len() - 1);
                self.set_output(out);
            }
        }
    }

    fn lint(&mut self) {
        match engine::lint(Path::new("."), self.profile_name()) {
            Ok(report) => {
                let mut out = vec![line(
                    format!(
                        "Lint — {} fichier(s), {} règle(s){}",
                        report.files_scanned,
                        report.rules_used,
                        report
                            .profile
                            .as_ref()
                            .map(|p| format!(", profil {p}"))
                            .unwrap_or_default()
                    ),
                    Level::Title,
                )];
                if report.violations.is_empty() {
                    out.push(line("✓ Aucune violation", Level::Ok));
                }
                for v in &report.violations {
                    let level = match v.severity.as_str() {
                        "error" => Level::Error,
                        "warning" => Level::Warning,
                        _ => Level::Info,
                    };
                    out.push(line(
                        format!("{}:{} [{}] {}", v.file, v.line, v.rule, v.message),
                        level,
                    ));
                }
                self.status = format!(
                    "Lint : {} violation(s) ({} erreur(s))",
                    report.violations.len(),
                    report.count("error")
                );
                self.set_output(out);
            }
            Err(e) => {
                self.status = "Lint en échec".into();
                self.set_output(vec![line(format!("✗ {e:#}"), Level::Error)]);
            }
        }
    }

    fn projects(&mut self) {
        let root = crate::paths::sites_dir();
        let list = scaffold::projects::list(&root);
        let mut out = vec![line(
            format!("Projets dans {}", root.display()),
            Level::Title,
        )];
        if list.is_empty() {
            out.push(line(
                "Aucun projet : choisissez « Générer un projet… »",
                Level::Info,
            ));
        }
        for p in &list {
            let git = p.git.as_ref().map_or(String::new(), |g| {
                format!(" · {}{}", g.branch, if g.dirty { " (modifié)" } else { "" })
            });
            out.push(line(
                format!(
                    "{} — {}{}{}",
                    p.name,
                    p.framework.label(),
                    p.package_manager
                        .as_deref()
                        .map_or(String::new(), |pm| format!(" · {pm}")),
                    git
                ),
                if p.installed {
                    Level::Ok
                } else {
                    Level::Warning
                },
            ));
            if !p.features.is_empty() {
                out.push(line(
                    format!("    briques : {}", p.features.join(", ")),
                    Level::Normal,
                ));
            }
            out.push(line(
                format!(
                    "    alpharust run {} <{}>",
                    p.name,
                    p.tasks.iter().map(|t| t.id).collect::<Vec<_>>().join("|")
                ),
                Level::Info,
            ));
        }
        self.status = format!("{} projet(s)", list.len());
        self.set_output(out);
    }

    fn doctor(&mut self) {
        let mut out = vec![line("Outils installés", Level::Title)];
        for f in Framework::ALL {
            let tools = scaffold::preflight(&Request::new(f, "doctor"));
            let ready = scaffold::preflight_check(&tools).is_ok();
            out.push(line(
                format!(
                    "{} — {}",
                    f.label(),
                    if ready { "prêt" } else { "outils manquants" }
                ),
                if ready { Level::Ok } else { Level::Warning },
            ));
            for t in tools {
                let mark = if t.ok {
                    "✔"
                } else if t.required {
                    "✘"
                } else {
                    "○"
                };
                out.push(line(
                    format!(
                        "  {} {:<28} {}",
                        mark,
                        t.label,
                        t.version.clone().unwrap_or_else(|| "—".into())
                    ),
                    if t.ok { Level::Normal } else { Level::Error },
                ));
                if !t.ok {
                    out.push(line(format!("      → {}", t.hint), Level::Info));
                }
            }
        }
        self.status = "Outils vérifiés".into();
        self.set_output(out);
    }

    /// Lance la génération dans un thread ; les événements arrivent via [`App::poll_job`].
    fn generate(&mut self, kind: Framework, name: &str) {
        let req = Request::new(kind, name);
        if let Err(e) = req.validate() {
            self.status = "Génération en échec".into();
            self.set_output(vec![line(format!("✗ {e:#}"), Level::Error)]);
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.cancel = Arc::new(AtomicBool::new(false));
        let cancel = Arc::clone(&self.cancel);
        std::thread::spawn(move || {
            let tx2 = tx.clone();
            let _ = scaffold::execute(
                &req,
                &mut |ev| {
                    let _ = tx2.send(ev);
                },
                &cancel,
            );
        });
        self.job = Some(rx);
        self.follow = true;
        self.scroll = 0;
        self.output = vec![line(
            format!("Génération {} « {} » (x : annuler)", kind.label(), name),
            Level::Title,
        )];
        self.status = "Génération en cours…".into();
    }

    /// Vide le canal d'événements de la génération en cours.
    pub fn poll_job(&mut self) {
        let Some(rx) = &self.job else { return };
        let mut events = Vec::new();
        let mut closed = false;
        loop {
            match rx.try_recv() {
                Ok(ev) => events.push(ev),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    closed = true;
                    break;
                }
            }
        }
        for ev in events {
            self.apply_event(ev);
        }
        if closed {
            self.job = None;
        }
    }

    pub fn apply_event(&mut self, ev: ScEvent) {
        match ev {
            ScEvent::Preflight { tools } => {
                for t in tools {
                    let mark = if t.ok { "✔" } else { "✘" };
                    self.output.push(line(
                        format!("{} {} {}", mark, t.label, t.version.unwrap_or_default()),
                        if t.ok { Level::Normal } else { Level::Error },
                    ));
                }
            }
            ScEvent::Plan { target, steps } => {
                self.output.push(line(
                    format!("{} étapes → {}", steps.len(), target),
                    Level::Info,
                ));
            }
            ScEvent::StepStart {
                index,
                total,
                label,
                command,
                ..
            } => {
                self.status = format!("[{}/{}] {}", index + 1, total, label);
                self.output.push(line(
                    format!("▶ [{}/{}] {}", index + 1, total, label),
                    Level::Title,
                ));
                if let Some(c) = command {
                    self.output.push(line(format!("  $ {c}"), Level::Info));
                }
            }
            ScEvent::Log { line: l, .. } => {
                self.output.push(line(format!("  │ {l}"), Level::Normal));
            }
            ScEvent::StepEnd {
                ok,
                fatal,
                millis,
                note,
                ..
            } => {
                let (mark, level) = if ok {
                    ("✔", Level::Ok)
                } else if fatal {
                    ("✘", Level::Error)
                } else {
                    ("⚠", Level::Warning)
                };
                self.output.push(line(
                    format!(
                        "  {} {:.1}s {}",
                        mark,
                        millis as f64 / 1000.0,
                        note.unwrap_or_default().lines().next().unwrap_or("")
                    ),
                    level,
                ));
            }
            ScEvent::Done { report } => {
                self.status = format!("Projet prêt : {}", report.target);
                self.output.push(line(report.summary(), Level::Ok));
                for s in &report.next_steps {
                    self.output.push(line(format!("  {s}"), Level::Info));
                }
            }
            ScEvent::Failed { message, .. } => {
                self.status = "Génération en échec".into();
                self.output.push(line(format!("✘ {message}"), Level::Error));
            }
        }
        if self.output.len() > 3000 {
            let extra = self.output.len() - 3000;
            self.output.drain(..extra);
        }
    }

    fn list_rules(&mut self) {
        match Rule::load_all() {
            Ok(rules) => {
                let rules = composer::compose(rules);
                let mut out = vec![line(format!("{} règles", rules.len()), Level::Title)];
                for r in &rules {
                    let level = match r.severity.as_str() {
                        "error" => Level::Error,
                        "warning" => Level::Warning,
                        _ => Level::Info,
                    };
                    out.push(line(
                        format!("{:<8} {:<7} {}", r.severity, r.kind(), r.id),
                        level,
                    ));
                }
                self.status = format!("{} règles chargées", rules.len());
                self.set_output(out);
            }
            Err(e) => self.set_output(vec![line(format!("✗ {e:#}"), Level::Error)]),
        }
    }

    fn list_profiles(&mut self) {
        let all = Rule::load_all().unwrap_or_default();
        let mut out = vec![line("Profils", Level::Title)];
        for name in &self.profiles {
            if let Ok(p) = Profile::load(name) {
                out.push(line(
                    format!("{:<12} {:>3} règles", name, p.select(all.clone()).len()),
                    Level::Normal,
                ));
            }
        }
        self.status = format!("{} profils", self.profiles.len());
        self.set_output(out);
    }

    pub fn draw(&self, f: &mut Frame) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(f.area());
        let title = Paragraph::new(format!(
            " AlphaRust TUI — profil : {}",
            self.profile_name().unwrap_or("toutes les règles")
        ))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, rows[0]);

        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(34), Constraint::Min(20)])
            .split(rows[1]);
        let items: Vec<ListItem> = MENU
            .iter()
            .enumerate()
            .map(|(i, (label, _))| {
                let style = if i == self.selected {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(format!(" {label} ")).style(style)
            })
            .collect();
        f.render_widget(
            List::new(items).block(Block::default().borders(Borders::ALL).title(" Menu ")),
            cols[0],
        );

        let lines: Vec<Line> = self
            .output
            .iter()
            .map(|l| {
                let style = match l.level {
                    Level::Normal => Style::default(),
                    Level::Ok => Style::default().fg(Color::Green),
                    Level::Error => Style::default().fg(Color::Red),
                    Level::Warning => Style::default().fg(Color::Yellow),
                    Level::Info => Style::default().fg(Color::Blue),
                    Level::Title => Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                };
                Line::from(Span::styled(l.text.clone(), style))
            })
            .collect();
        let inner_h = cols[1].height.saturating_sub(2) as usize;
        let scroll = if self.follow {
            lines.len().saturating_sub(inner_h).min(u16::MAX as usize) as u16
        } else {
            self.scroll
        };
        f.render_widget(
            Paragraph::new(lines)
                .scroll((scroll, 0))
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title(" Résultat ")),
            cols[1],
        );

        let footer = match &self.mode {
            Mode::Menu => format!(
                "↑↓ naviguer · Entrée choisir · p profil · PgUp/PgDn défiler · x annuler · q quitter │ {}",
                self.status
            ),
            Mode::Input { kind, buffer } => format!(
                "Nom du projet {} : {}█  (Entrée valider · Échap annuler)",
                kind.label(),
                buffer
            ),
        };
        f.render_widget(
            Paragraph::new(footer).block(Block::default().borders(Borders::ALL)),
            rows[2],
        );
    }
}

/// Restaure le terminal même en cas d'erreur ou de panique.
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

pub fn run() -> Result<()> {
    if !io::stdout().is_terminal() || !io::stdin().is_terminal() {
        bail!("la TUI nécessite un terminal interactif (stdin/stdout ne sont pas des TTY)");
    }
    enable_raw_mode()?;
    let _guard = TerminalGuard;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let mut app = App::new();
    while !app.quit {
        app.poll_job();
        terminal.draw(|f| app.draw(f))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }
    }
    Ok(())
}

/// Rend l'application dans un buffer texte (tests, documentation).
pub fn render_to_string(app: &App, width: u16, height: u16) -> String {
    let backend = ratatui::backend::TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("terminal de test");
    terminal.draw(|f| app.draw(f)).expect("rendu");
    let buffer = terminal.backend().buffer().clone();
    (0..buffer.area.height)
        .map(|y| {
            (0..buffer.area.width)
                .map(|x| buffer[(x, y)].symbol().to_string())
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn navigation_is_bounded() {
        let mut app = App::new();
        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.selected, 0);
        for _ in 0..30 {
            app.handle_key(key(KeyCode::Down));
        }
        assert_eq!(app.selected, MENU.len() - 1);
    }

    #[test]
    fn quit_keys() {
        for k in [KeyCode::Char('q'), KeyCode::Esc] {
            let mut app = App::new();
            app.handle_key(key(k));
            assert!(app.quit);
        }
        let mut app = App::new();
        app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert!(app.quit);
    }

    #[test]
    fn list_rules_shows_real_rules() {
        let mut app = App::new();
        app.activate(Action::ListRules);
        assert!(app.output.len() > 50);
        assert!(app.output.iter().any(|l| l.text.contains("core/no-eval")));
        let screen = render_to_string(&app, 100, 30);
        assert!(screen.contains("AlphaRust TUI"));
        assert!(screen.contains("Résultat"));
    }

    #[test]
    fn profile_cycles_and_wraps() {
        let mut app = App::new();
        let n = app.profiles.len();
        assert!(n >= 6);
        for i in 0..n {
            app.handle_key(key(KeyCode::Char('p')));
            assert_eq!(app.profile, Some(i));
        }
        app.handle_key(key(KeyCode::Char('p')));
        assert_eq!(app.profile, None);
    }

    #[test]
    fn lint_uses_the_engine() {
        let mut app = App::new();
        app.activate(Action::Lint);
        assert!(
            app.output[0].text.starts_with("Lint —"),
            "{:?}",
            app.output[0]
        );
    }

    #[test]
    fn generate_prompts_then_rejects_bad_name() {
        let mut app = App::new();
        app.activate(Action::Generate(Framework::Wagtail));
        assert!(matches!(app.mode, Mode::Input { .. }));
        for _ in 0..40 {
            app.handle_key(key(KeyCode::Backspace));
        }
        for c in "../evil".chars() {
            app.handle_key(key(KeyCode::Char(c)));
        }
        app.handle_key(key(KeyCode::Enter));
        assert_eq!(app.mode, Mode::Menu);
        assert!(
            app.output[0].text.contains("invalide"),
            "{:?}",
            app.output[0]
        );
    }

    #[test]
    fn job_events_are_rendered_in_the_result_pane() {
        use crate::scaffold::Phase;
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.job = Some(rx);
        app.follow = true;
        app.output.clear();
        tx.send(ScEvent::StepStart {
            index: 0,
            total: 2,
            id: "create".into(),
            label: "Créer le projet".into(),
            phase: Phase::Scaffold,
            command: Some("npm create astro@5.3.2 demo".into()),
        })
        .unwrap();
        tx.send(ScEvent::Log {
            index: 0,
            line: "Dependencies installed".into(),
        })
        .unwrap();
        tx.send(ScEvent::StepEnd {
            index: 0,
            id: "create".into(),
            ok: true,
            fatal: true,
            millis: 1500,
            note: None,
        })
        .unwrap();
        tx.send(ScEvent::Failed {
            message: "boum".into(),
            step: None,
        })
        .unwrap();
        drop(tx);
        assert!(app.running());
        app.poll_job();
        assert!(!app.running(), "le canal fermé termine la génération");
        let text: Vec<&str> = app.output.iter().map(|l| l.text.as_str()).collect();
        assert!(text.iter().any(|l| l.contains("[1/2] Créer le projet")));
        assert!(text
            .iter()
            .any(|l| l.contains("$ npm create astro@5.3.2 demo")));
        assert!(text.iter().any(|l| l.contains("Dependencies installed")));
        assert!(text.iter().any(|l| l.contains("✔ 1.5s")));
        assert!(text.iter().any(|l| l.contains("✘ boum")));
        let screen = render_to_string(&app, 100, 24);
        assert!(screen.contains("boum"), "{screen}");
    }

    #[test]
    fn projects_action_lists_generated_projects() {
        let mut app = App::new();
        app.activate(Action::Projects);
        assert!(app.status.ends_with("projet(s)"), "{}", app.status);
    }

    #[test]
    fn doctor_lists_tools_for_each_framework() {
        let mut app = App::new();
        app.activate(Action::Doctor);
        let all: String = app
            .output
            .iter()
            .map(|l| l.text.clone())
            .collect::<Vec<_>>()
            .join("\n");
        for f in ["Astro", "Laravel", "Wagtail", "Rust"] {
            assert!(all.contains(f), "{all}");
        }
        assert!(all.contains("Node.js") && all.contains("Cargo"));
    }

    #[test]
    fn escape_cancels_input() {
        let mut app = App::new();
        app.activate(Action::Generate(Framework::Laravel));
        app.handle_key(key(KeyCode::Esc));
        assert_eq!(app.mode, Mode::Menu);
        assert!(!app.quit);
    }

    #[test]
    fn render_menu_highlights_selection() {
        let mut app = App::new();
        app.handle_key(key(KeyCode::Down));
        let screen = render_to_string(&app, 100, 24);
        assert!(screen.contains("Générer un projet Astro"));
        assert!(screen.contains("Quitter"));
    }
}
