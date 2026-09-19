use crate::core::Article;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    ArticleList,
    Quitting,
}

pub struct App {
    pub articles: Vec<Article>,
    pub selected: usize,
    pub screen: Screen,
    pub status: String,
}

impl App {
    pub fn new(articles: Vec<Article>) -> Self {
        Self {
            articles,
            selected: 0,
            screen: Screen::ArticleList,
            status: String::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.screen {
            Screen::ArticleList => self.handle_list_key(key),
            Screen::Quitting => false,
        }
    }

    fn handle_list_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.screen = Screen::Quitting;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous();
            }
            KeyCode::Char('g') | KeyCode::Home => {
                self.selected = 0;
            }
            KeyCode::Char('G') | KeyCode::End => {
                self.selected = self.articles.len().saturating_sub(1);
            }
            KeyCode::Char('b') => {
                // TODO: بعداً به repository وصل میشه
                self.status = format!("bookmarked: {}", self.current_title());
            }
            _ => return false,
        }
        true
    }

    fn next(&mut self) {
        if self.selected < self.articles.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    fn previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn current_title(&self) -> &str {
        self.articles
            .get(self.selected)
            .map(|a| a.title.as_str())
            .unwrap_or("")
    }
}