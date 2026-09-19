use crate::core::Article;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid feed format: {0}")]
    InvalidFormat(#[from] feed_rs::parser::ParseFeedError),
    #[error("feed contains no entries")]
    EmptyFeed,
}

pub fn parse_feed(xml: &str, feed_id: i64) -> Result<Vec<Article>, ParseError> {
    let parsed = feed_rs::parser::parse(xml.as_bytes())?;

    if parsed.entries.is_empty() {
        return Err(ParseError::EmptyFeed);
    }

    Ok(parsed
        .entries
        .into_iter()
        .map(|entry| Article {
            id: None,
            feed_id,
            guid: entry.id,
            title: entry.title.map(|t| t.content).unwrap_or_default(),
            link: entry.links.first().map(|l| l.href.clone()).unwrap_or_default(),
            summary: entry.summary.map(|s| s.content),
            content: entry.content.and_then(|c| c.body),
            author: entry.authors.first().map(|p| p.name.clone()),
            published_at: entry.published.or(entry.updated).map(|d| d.into()),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RSS: &str = r#"<?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <title>Test Feed</title>
            <item>
                <guid>abc-123</guid>
                <title>First Article</title>
                <link>https://example.com/1</link>
                <description>Some summary</description>
                <pubDate>Tue, 10 Jun 2025 10:00:00 GMT</pubDate>
            </item>
            <item>
                <guid>abc-456</guid>
                <title>Second Article</title>
                <link>https://example.com/2</link>
            </item>
        </channel>
    </rss>"#;

    #[test]
    fn parses_rss_correctly() {
        let articles = parse_feed(SAMPLE_RSS, 1).unwrap();
        assert_eq!(articles.len(), 2);
        assert_eq!(articles[0].title, "First Article");
        assert_eq!(articles[0].link, "https://example.com/1");
        assert_eq!(articles[0].feed_id, 1);
        assert!(articles[0].published_at.is_some());
        assert!(articles[1].published_at.is_none());
    }

    #[test]
    fn rejects_invalid_xml() {
        assert!(parse_feed("<not-a-feed>", 1).is_err());
    }
}