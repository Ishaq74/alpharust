//! Interface terminal (ratatui) : menu, lint, génération de projets, listes.
//!
//! L'état ([`App`]) est indépendant du terminal : les tests pilotent `handle_key`
//! et rendent dans un `TestBackend`.

use crate::engine::{self, composer};
use crate::generators::{self, Kind};
use crate::parser::Rule;
use crate::profiles::Profile;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Lint,
    Generate(Kind),
    ListRules,
    ListProfiles,
    ListCheckers,
    Quit,
}

pub const MENU: [(&str, Action); 9] = [
    ("Lint du répertoire courant", Action::Lint),
    ("Générer un projet Wagtail", Action::Generate(Kind::Wagtail)),
    ("Générer un projet Astro", Action::Generate(Kind::Astro)),
    ("Générer un projet Laravel", Action::Generate(Kind::Laravel)),
    (
        "Initialiser un projet Rust",
        Action::Generate(Kind::Bootstrap),
    ),
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
        kind: Kind,
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
            KeyCode::PageDown => self.scroll = self.scroll.saturating_add(8),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(8),
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
                let default = match kind {
                    Kind::Bootstrap => "rust-app".to_string(),
                    k => format!("{}-site", k.label().to_lowercase()),
                };
                self.mode = Mode::Input {
                    kind,
                    buffer: default,
                };
                self.status = "Nom du projet (Entrée : valider, Échap : annuler)".into();
            }
            Action::ListRules => self.list_rules(),
            Action::ListProfiles => self.list_profiles(),
            Action::ListCheckers => {
                let mut out = vec![line("Checkers sémantiques", Level::Title)];
                for (n, d) in crate::checkers::catalog() {
                    out.push(line(format!("{:<20} {}", n, d), Level::Normal));
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
                            .map(|p| format!(", profil {}", p))
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
                self.set_output(vec![line(format!("✗ {:#}", e), Level::Error)]);
            }
        }
    }

    fn generate(&mut self, kind: Kind, name: &str) {
        let target = generators::default_target(name);
        match generators::generate(kind, &target, name, false) {
            Ok(g) => {
                let mut out = vec![line(g.summary(), Level::Ok)];
                out.extend(
                    g.files
                        .iter()
                        .map(|f| line(format!("  + {}", f), Level::Normal)),
                );
                self.status = format!("{} généré dans sites/{}", kind.label(), name);
                self.set_output(out);
            }
            Err(e) => {
                self.status = "Génération en échec".into();
                self.set_output(vec![line(format!("✗ {:#}", e), Level::Error)]);
            }
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
            Err(e) => self.set_output(vec![line(format!("✗ {:#}", e), Level::Error)]),
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
                ListItem::new(format!(" {} ", label)).style(style)
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
        f.render_widget(
            Paragraph::new(lines)
                .scroll((self.scroll, 0))
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title(" Résultat ")),
            cols[1],
        );

        let footer = match &self.mode {
            Mode::Menu => format!(
                "↑↓ naviguer · Entrée choisir · p profil · PgUp/PgDn défiler · q quitter │ {}",
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
        app.activate(Action::Generate(Kind::Wagtail));
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
    fn escape_cancels_input() {
        let mut app = App::new();
        app.activate(Action::Generate(Kind::Laravel));
        app.handle_key(key(KeyCode::Esc));
        assert_eq!(app.mode, Mode::Menu);
        assert!(!app.quit);
    }

    #[test]
    fn render_menu_highlights_selection() {
        let mut app = App::new();
        app.handle_key(key(KeyCode::Down));
        let screen = render_to_string(&app, 100, 24);
        assert!(screen.contains("Générer un projet Wagtail"));
        assert!(screen.contains("Quitter"));
    }
}
