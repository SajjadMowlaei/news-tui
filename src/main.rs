use anyhow::Result;
use news_tui::config::AppConfig;
use news_tui::core::Feed;
use news_tui::fetcher::Fetcher;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;
    println!("✓ Loaded {} feeds from config", config.feeds.len());

    let feeds: Vec<Feed> = config.feeds.into_iter().map(Feed::from).collect();

    let fetcher = Fetcher::new()?;

    for (feed, result) in fetcher.fetch_all(&feeds).await {
        match result {
            Ok(body) => println!("✓ {:<20} → {} bytes", feed.name, body.len()),
            Err(e) => eprintln!("✗ {:<20} → {}", feed.name, e),
        }
    }

    Ok(())
}