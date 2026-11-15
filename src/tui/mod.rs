use anyhow::Result;
use crossterm::{event, terminal};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

pub fn run() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
                .split(f.size());

            let title = Paragraph::new("AlphaRust TUI")
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let items = vec![
                ListItem::new("1. Lint project"),
                ListItem::new("2. Generate Wagtail"),
                ListItem::new("3. Generate Astro"),
                ListItem::new("4. Generate Laravel"),
                ListItem::new("5. List rules"),
                ListItem::new("q. Quit"),
            ];
            let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Menu"));
            f.render_widget(list, chunks[1]);

            let help = Paragraph::new("AlphaRust — TUI mode (enhanced forge style)")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(help, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
