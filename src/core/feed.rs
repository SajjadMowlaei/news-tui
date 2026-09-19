#[derive(Debug, Clone)]
pub struct Feed {
    pub id: Option<i64>,
    pub name: String,
    pub url: String,
    pub category: Option<String>,
    pub country: Option<String>,
}