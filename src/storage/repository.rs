use crate::core::{Article, Feed};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};

pub struct Repository {
    conn: Connection,
}

impl Repository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)
            .with_context(|| format!("cannot open database: {db_path}"))?;
        conn.execute_batch(super::schema::INIT_SQL)?;
        Ok(Self { conn })
    }

    pub fn upsert_feed(&self, feed: &Feed) -> Result<i64> {
        self.conn
            .execute(
                "INSERT INTO feeds (name, url, category, country)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(url) DO UPDATE SET
                    name = excluded.name,
                    category = excluded.category,
                    country = excluded.country",
                params![feed.name, feed.url, feed.category, feed.country],
            )
            .context("failed to upsert feed")?;

        Ok(self.conn.query_row(
            "SELECT id FROM feeds WHERE url = ?1",
            params![feed.url],
            |row| row.get(0),
        )?)
    }

    pub fn list_feeds(&self) -> Result<Vec<Feed>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, url, category, country FROM feeds ORDER BY name",
        )?;

        let feeds = stmt
            .query_map([], |row| {
                Ok(Feed {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    url: row.get(2)?,
                    category: row.get(3)?,
                    country: row.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(feeds)
    }

    pub fn save_articles(&self, articles: &[Article]) -> Result<()> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO articles (feed_id, guid, title, link, summary, content, author, published_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(feed_id, guid) DO UPDATE SET
                title = excluded.title,
                summary = excluded.summary,
                content = excluded.content
             ",
        )?;

        for a in articles {
            stmt.execute(params![
                a.feed_id,
                a.guid,
                a.title,
                a.link,
                a.summary,
                a.content,
                a.author,
                a.published_at.map(|d| d.to_rfc3339()),
            ])?;
        }

        Ok(())
    }

    pub fn list_articles(&self, feed_id: Option<i64>) -> Result<Vec<Article>> {
        let (sql, param_value): (String, Option<i64>) = match feed_id {
            Some(id) => (
                "SELECT id, feed_id, guid, title, link, summary, content, author, published_at
                 FROM articles WHERE feed_id = ?1
                 ORDER BY published_at DESC".into(),
                Some(id),
            ),
            None => (
                "SELECT id, feed_id, guid, title, link, summary, content, author, published_at
                 FROM articles
                 ORDER BY published_at DESC".into(),
                None,
            ),
        };

        let mut stmt = self.conn.prepare(&sql)?;
        let map_row = |row: &rusqlite::Row| -> rusqlite::Result<Article> {
            let published_at: Option<String> = row.get(8)?;
            Ok(Article {
                id: Some(row.get(0)?),
                feed_id: row.get(1)?,
                guid: row.get(2)?,
                title: row.get(3)?,
                link: row.get(4)?,
                summary: row.get(5)?,
                content: row.get(6)?,
                author: row.get(7)?,
                published_at: published_at
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|d| d.with_timezone(&Utc)),
            })
        };

        let articles = match param_value {
            Some(id) => stmt
                .query_map(params![id], map_row)?
                .collect::<std::result::Result<Vec<_>, _>>()?,
            None => stmt
                .query_map([], map_row)?
                .collect::<std::result::Result<Vec<_>, _>>()?,
        };

        Ok(articles)
    }

    pub fn toggle_bookmark(&self, article_id: i64) -> Result<bool> {
        self.toggle_flag(article_id, "bookmarked")
    }

    pub fn toggle_like(&self, article_id: i64) -> Result<bool> {
        self.toggle_flag(article_id, "liked")
    }

    fn toggle_flag(&self, article_id: i64, column: &str) -> Result<bool> {
        self.conn.execute(
            &format!("UPDATE articles SET {column} = 1 - {column} WHERE id = ?1"),
            params![article_id],
        )?;

        Ok(self.conn.query_row(
            &format!("SELECT {column} FROM articles WHERE id = ?1"),
            params![article_id],
            |row| row.get(0),
        )?)
    }

    pub fn mark_read(&self, article_id: i64, percent: u8) -> Result<()> {
        let percent = percent.min(100);
        self.conn.execute(
            "UPDATE articles
             SET read_percent = ?2,
                 read = CASE WHEN ?2 >= 90 THEN 1 ELSE read END
             WHERE id = ?1",
            params![article_id, percent as i64],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_repo() -> Repository {
        Repository::new(":memory:").unwrap()
    }

    #[test]
    fn feed_upsert_is_idempotent() {
        let repo = test_repo();
        let feed = Feed {
            id: None,
            name: "Test".into(),
            url: "https://example.com/rss".into(),
            category: None,
            country: Some("iran".into()),
        };

        let id1 = repo.upsert_feed(&feed).unwrap();
        let id2 = repo.upsert_feed(&feed).unwrap();

        assert_eq!(id1, id2);
        assert_eq!(repo.list_feeds().unwrap().len(), 1);
    }

    #[test]
    fn duplicate_articles_are_updated_not_duplicated() {
        let repo = test_repo();
        let feed_id = repo.upsert_feed(&Feed {
            id: None,
            name: "T".into(),
            url: "u".into(),
            category: None,
            country: None,
        }).unwrap();

        let make = |title: &str| Article {
            id: None,
            feed_id,
            guid: "guid-1".into(),
            title: title.into(),
            link: "https://x.com/1".into(),
            summary: None,
            content: None,
            author: None,
            published_at: None,
        };

        repo.save_articles(&[make("v1")]).unwrap();
        repo.save_articles(&[make("v2")]).unwrap();

        let all = repo.list_articles(None).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].title, "v2");
    }

    #[test]
    fn toggle_bookmark_works() {
        let repo = test_repo();
        let feed_id = repo.upsert_feed(&Feed {
            id: None, name: "T".into(), url: "u".into(),
            category: None, country: None,
        }).unwrap();

        repo.save_articles(&[Article {
            id: None, feed_id, guid: "g".into(), title: "t".into(),
            link: "l".into(), summary: None, content: None,
            author: None, published_at: None,
        }]).unwrap();

        let article_id = repo.list_articles(None).unwrap()[0].id.unwrap();

        assert!(repo.toggle_bookmark(article_id).unwrap());   // روشن شد
        assert!(!repo.toggle_bookmark(article_id).unwrap());  // خاموش شد
    }
}