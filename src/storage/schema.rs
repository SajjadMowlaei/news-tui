pub const INIT_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS feeds (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    name     TEXT NOT NULL,
    url      TEXT NOT NULL UNIQUE,
    category TEXT,
    country  TEXT
);

CREATE TABLE IF NOT EXISTS articles (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    feed_id      INTEGER NOT NULL REFERENCES feeds(id),
    guid         TEXT NOT NULL,
    title        TEXT NOT NULL,
    link         TEXT NOT NULL,
    summary      TEXT,
    content      TEXT,
    author       TEXT,
    published_at TEXT,
    liked        INTEGER NOT NULL DEFAULT 0,
    read         INTEGER NOT NULL DEFAULT 0,
    read_percent INTEGER NOT NULL DEFAULT 0,
    bookmarked   INTEGER NOT NULL DEFAULT 0,
    fetched_at   TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(feed_id, guid)
);

CREATE INDEX IF NOT EXISTS idx_articles_feed ON articles(feed_id);
CREATE INDEX IF NOT EXISTS idx_articles_published ON articles(published_at);
"#;