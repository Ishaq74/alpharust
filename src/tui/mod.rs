use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected = 0usize;
    let mut status = String::from("Prêt");
    let menu = [
        "Lint current directory",
        "Generate Wagtail",
        "Generate Astro",
        "Generate Laravel",
        "List rules",
        "Quit",
    ];

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(area);

            let title = Paragraph::new("AlphaRust TUI")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let items: Vec<ListItem> = menu
                .iter()
                .enumerate()
                .map(|(i, text)| {
                    let style = if i == selected {
                        Style::default().fg(Color::Black).bg(Color::Yellow)
                    } else {
                        Style::default()
                    };
                    ListItem::new(*text).style(style)
                })
                .collect();

            let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Menu"));
            f.render_widget(list, chunks[1]);

            let footer = Paragraph::new(format!("↑↓ move | Enter select | q quit | {}", status))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        if selected + 1 < menu.len() {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        match selected {
                            0 => {
                                let rules = crate::parser::Rule::load_all().unwrap_or_default();
                                let v = crate::engine::runner::run(&rules, std::path::Path::new("."))
                                    .unwrap_or_default();
                                status = format!("Lint: {} violations", v.len());
                            }
                            1 => {
                                let p = PathBuf::from("sites/wagtail-site");
                                let _ = crate::generators::wagtail::generate(&p, "wagtail-site");
                                status = "Wagtail généré dans sites/".into();
                            }
                            2 => {
                                let p = PathBuf::from("sites/astro-site");
                                let _ = crate::generators::astro::generate(&p, "astro-site");
                                status = "Astro généré dans sites/".into();
                            }
                            3 => {
                                let p = PathBuf::from("sites/laravel-site");
                                let _ = crate::generators::laravel::generate(&p, "laravel-site");
                                status = "Laravel généré dans sites/".into();
                            }
                            4 => {
                                let rules = crate::parser::Rule::load_all().unwrap_or_default();
                                status = format!("{} règles chargées", rules.len());
                            }
                            5 => break,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
