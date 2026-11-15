use anyhow::Result;
use crossterm::{event::{self, Event, KeyCode}, terminal};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io::{self, Stdout};

pub fn run() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected = 0;
    let menu = [
        "Lint current directory",
        "Generate Wagtail site",
        "Generate Astro site",
        "Generate Laravel site",
        "List rules",
        "Quit",
    ];

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
                .split(f.size());

            let title = Paragraph::new(" AlphaRust TUI ")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let items: Vec<ListItem> = menu
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let style = if i == selected {
                        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };
                    ListItem::new(*m).style(style)
                })
                .collect();

            let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Menu "));
            f.render_widget(list, chunks[1]);

            let help = Paragraph::new(" ↑↓ navigate | Enter select | q quit ")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(help, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < menu.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        match selected {
                            0 => { let _ = crate::engine::runner::run(&crate::parser::Rule::load_all().unwrap_or_default(), std::path::Path::new(".")); }
                            1 => { let _ = crate::generators::wagtail::generate(&std::path::PathBuf::from("sites/wagtail-site"), "wagtail-site"); }
                            2 => { let _ = crate::generators::astro::generate(&std::path::PathBuf::from("sites/astro-site"), "astro-site"); }
                            3 => { let _ = crate::generators::laravel::generate(&std::path::PathBuf::from("sites/laravel-site"), "laravel-site"); }
                            4 => { let _ = crate::parser::Rule::load_all(); }
                            5 => break,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    Ok(())
}
