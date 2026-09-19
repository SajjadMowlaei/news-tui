use anyhow::Result;
use news_tui::config::AppConfig;
use news_tui::core::Feed;
use news_tui::fetcher::Fetcher;
use news_tui::parser::rss;
use news_tui::storage::repository::Repository;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;

    let db_path = AppConfig::db_path()?;
    let repo = Repository::new(db_path.to_str().unwrap())?;
    println!("✓ Database: {}", db_path.display());

    let mut feeds = Vec::new();
    for fc in &config.feeds {
        let feed = Feed::from(fc.clone());
        let id = repo.upsert_feed(&feed)?;
        feeds.push((id, feed));
    }
    println!("✓ Synced {} feeds", feeds.len());

    let fetcher = Fetcher::new()?;
    for (id, feed) in &feeds {
        match fetcher.fetch(feed).await {
            Ok(body) => match rss::parse_feed(&body, *id) {
                Ok(articles) => {
                    repo.save_articles(&articles)?;
                    println!("✓ {:<20} → {} articles saved", feed.name, articles.len());
                }
                Err(e) => eprintln!("✗ {:<20} → parse: {}", feed.name, e),
            },
            Err(e) => eprintln!("✗ {:<20} → fetch: {}", feed.name, e),
        }
    }

    let all = repo.list_articles(None)?;
    println!("\n─── Latest articles ───");
    for a in all.iter().take(5) {
        println!("• {}", a.title);
    }
    println!("... total: {} articles in DB", all.len());

    Ok(())
}