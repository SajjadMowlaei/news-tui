pub mod app;
pub mod views;

use anyhow::{Result};
use app::{App, Screen};
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

pub fn run(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(app.selected));

    let result = event_loop(&mut terminal, app, &mut list_state);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn event_loop(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,
    app: &mut App,
    list_state: &mut ratatui::widgets::ListState,
) -> Result<()> {
    loop {
        terminal.draw(|f| views::article_list::render(f, app, list_state))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key);
                    list_state.select(Some(app.selected));
                }
            }
        }

        if app.screen == Screen::Quitting {
            return Ok(());
        }
    }
}