use crate::core::Feed;
use anyhow::Result;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("feed returned empty body: {url}")]
    EmptyBody { url: String },
}

pub struct Fetcher {
    client: reqwest::Client,
}

impl Fetcher {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent("terminal-news-tui/0.1")
            .build()?;
        Ok(Self { client })
    }

    pub async fn fetch(&self, feed: &Feed) -> Result<String> {
        let body = self.client.get(&feed.url).send().await?.error_for_status()?.text().await?;

        if body.trim().is_empty() {
            return Err(FetchError::EmptyBody {
                url: feed.url.clone(),
            }
            .into());
        }

        Ok(body)
    }

    pub async fn fetch_all(&self, feeds: &[Feed]) -> Vec<(Feed, Result<String>)> {
        let futures = feeds.iter().map(|feed| {
            let feed = feed.clone();
            async move {
                let result = self.fetch(&feed).await;
                (feed, result)
            }
        });

        futures::future::join_all(futures).await
    }
}