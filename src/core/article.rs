use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Article {
    pub id: Option<i64>,
    pub feed_id: i64,
    pub guid: String,
    pub title: String,
    pub link: String,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}