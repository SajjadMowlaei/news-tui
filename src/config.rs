use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use crate::core::Feed;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub feeds: Vec<FeedConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeedConfig {
    pub name: String,
    pub url: String,
    pub category: Option<String>,
    pub country: Option<String>,
}

impl From<FeedConfig> for Feed {
    fn from(fc: FeedConfig) -> Self {
        Feed {
            id: None,
            name: fc.name,
            url: fc.url,
            category: fc.category,
            country: fc.country,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let path = Self::resolve_path()?;
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config: {}", path.display()))?;
        toml::from_str(&content).context("failed to parse feeds.toml")
    }

    fn resolve_path() -> Result<PathBuf> {
        let local = PathBuf::from("config/feeds.toml");
        if local.exists() {
            return Ok(local);
        }

        let config_dir = dirs::config_dir()
            .context("cannot resolve system config directory")?
            .join("news-tui")
            .join("feeds.toml");

        Ok(config_dir)
    }
}