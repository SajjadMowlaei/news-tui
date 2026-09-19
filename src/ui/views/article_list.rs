use crate::ui::app::App;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, list_state: &mut ListState) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(f.area());

    let header_text = Line::from(format!(
        " News Reader │ {} articles │ {} ",
        app.articles.len(),
        app.status
    ))
    .style(Style::default().bg(Color::DarkGray).bold());

    f.render_widget(Paragraph::new(header_text), header);

    let items: Vec<ListItem> = app
        .articles
        .iter()
        .map(|a| {
            ListItem::new(Line::from(format!(" {}", a.title)))
                .style(Style::default().fg(Color::White))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Articles "))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    f.render_stateful_widget(list, body, list_state);
}