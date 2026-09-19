use anyhow::Result;
use news_tui::config::AppConfig;
use news_tui::core::Feed;
use news_tui::fetcher::Fetcher;
use news_tui::parser::rss;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;
    println!("✓ Loaded {} feeds from config", config.feeds.len());

    let feeds: Vec<Feed> = config.feeds.into_iter().map(Feed::from).collect();
    let fetcher = Fetcher::new()?;

    for (idx, (feed, result)) in fetcher.fetch_all(&feeds).await.into_iter().enumerate() {
        match result {
            Ok(body) => match rss::parse_feed(&body, idx as i64) {
                Ok(articles) => {
                    println!(
                        "✓ {:<20} → {} articles",
                        feed.name,
                        articles.len()
                    );
                    if let Some(first) = articles.first() {
                        println!("    └─ {}", first.title);
                    }
                }
                Err(e) => eprintln!("✗ {:<20} → parse error: {}", feed.name, e),
            },
            Err(e) => eprintln!("✗ {:<20} → fetch error: {}", feed.name, e),
        }
    }

    Ok(())
}