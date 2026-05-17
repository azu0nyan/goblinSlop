use rusqlite::{Connection, Result as SqlResult};

#[cfg(test)]
use crate::domain::DynamicPage;
use crate::domain::{ContentEntry, SourceRef};

// ============================================================
// Schema
// ============================================================

pub fn init_db(path: &str) -> SqlResult<Connection> {
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "PRAGMA foreign_keys = OFF;

         DROP TABLE IF EXISTS content_tags;
         DROP TABLE IF EXISTS content_references;
         DROP TABLE IF EXISTS content_sources;
         DROP TABLE IF EXISTS dynamic_pages;
         DROP TABLE IF EXISTS content;
         DROP TABLE IF EXISTS keywords;

         CREATE TABLE content (
             id              INTEGER PRIMARY KEY AUTOINCREMENT,
             slug            TEXT NOT NULL UNIQUE,
             title           TEXT NOT NULL,
             body_markdown   TEXT NOT NULL,
             body_html       TEXT NOT NULL,
             category        TEXT NOT NULL DEFAULT 'general',
             is_dynamic      INTEGER NOT NULL DEFAULT 0,
             date_added      TEXT NOT NULL DEFAULT '1970-01-01T00:00:00Z',
             image           TEXT DEFAULT NULL
         );

         CREATE TABLE content_tags (
             content_id  INTEGER NOT NULL,
             tag         TEXT NOT NULL,
             PRIMARY KEY (content_id, tag),
             FOREIGN KEY (content_id) REFERENCES content(id)
         );

         CREATE TABLE content_references (
             content_id  INTEGER NOT NULL,
             ref_slug    TEXT NOT NULL,
             PRIMARY KEY (content_id, ref_slug),
             FOREIGN KEY (content_id) REFERENCES content(id)
         );

         CREATE TABLE content_sources (
             id          INTEGER PRIMARY KEY AUTOINCREMENT,
             content_id  INTEGER NOT NULL,
             source_name TEXT NOT NULL,
             source_url  TEXT NOT NULL DEFAULT '',
             FOREIGN KEY (content_id) REFERENCES content(id)
         );

         CREATE TABLE dynamic_pages (
             path TEXT PRIMARY KEY,
             title TEXT NOT NULL,
             content TEXT NOT NULL,
             keywords TEXT NOT NULL DEFAULT ''
         );

         CREATE INDEX IF NOT EXISTS idx_content_category ON content(category);
         CREATE INDEX IF NOT EXISTS idx_content_tags_tag ON content_tags(tag);
         CREATE INDEX IF NOT EXISTS idx_content_tags_cid ON content_tags(content_id);",
    )?;

    Ok(conn)
}

// ============================================================
// Inserts
// ============================================================

pub fn insert_content(conn: &Connection, entry: &ContentEntry) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO content (slug, title, body_markdown, body_html, category, is_dynamic, date_added, image)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            entry.slug,
            entry.title,
            entry.body_markdown,
            entry.body_html,
            entry.category,
            entry.is_dynamic as i32,
            entry.date_added,
            entry.image,
        ],
    )?;
    let content_id = conn.last_insert_rowid();

    for tag in &entry.tags {
        conn.execute(
            "INSERT OR IGNORE INTO content_tags (content_id, tag) VALUES (?1, ?2)",
            rusqlite::params![content_id, tag],
        )?;
    }

    for ref_slug in &entry.references {
        conn.execute(
            "INSERT OR IGNORE INTO content_references (content_id, ref_slug) VALUES (?1, ?2)",
            rusqlite::params![content_id, ref_slug],
        )?;
    }

    for src in &entry.sources {
        conn.execute(
            "INSERT INTO content_sources (content_id, source_name, source_url) VALUES (?1, ?2, ?3)",
            rusqlite::params![content_id, src.name, src.url],
        )?;
    }

    Ok(content_id)
}

// ============================================================
// Queries
// ============================================================

const SELECT_CONTENT: &str = "SELECT id, slug, title, body_markdown, body_html, category, is_dynamic, date_added, image FROM content";

fn load_tags(conn: &Connection, content_id: i64) -> SqlResult<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT tag FROM content_tags WHERE content_id = ?1 ORDER BY tag")?;
    stmt.query_map(rusqlite::params![content_id], |row| row.get(0))?
        .collect()
}

fn load_references(conn: &Connection, content_id: i64) -> SqlResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT ref_slug FROM content_references WHERE content_id = ?1 ORDER BY ref_slug",
    )?;
    stmt.query_map(rusqlite::params![content_id], |row| row.get(0))?
        .collect()
}

fn load_sources(conn: &Connection, content_id: i64) -> SqlResult<Vec<SourceRef>> {
    let mut stmt = conn.prepare(
        "SELECT source_name, source_url FROM content_sources WHERE content_id = ?1 ORDER BY id",
    )?;
    stmt.query_map(rusqlite::params![content_id], |row| {
        Ok(SourceRef {
            name: row.get(0)?,
            url: row.get(1)?,
        })
    })?
    .collect()
}

fn hydrate_row(conn: &Connection, row: &rusqlite::Row) -> SqlResult<ContentEntry> {
    let id: i64 = row.get(0)?;
    let image_raw: Option<String> = row.get(8)?;
    Ok(ContentEntry {
        id,
        slug: row.get(1)?,
        title: row.get(2)?,
        body_markdown: row.get(3)?,
        body_html: row.get(4)?,
        category: row.get(5)?,
        tags: load_tags(conn, id)?,
        references: load_references(conn, id)?,
        sources: load_sources(conn, id)?,
        is_dynamic: row.get::<_, i32>(6)? != 0,
        date_added: row.get(7)?,
        image: image_raw.filter(|s| !s.is_empty()),
    })
}

pub fn get_content_by_slug(conn: &Connection, slug: &str) -> SqlResult<Option<ContentEntry>> {
    let mut stmt = conn.prepare(&format!("{SELECT_CONTENT} WHERE slug = ?1"))?;
    let mut rows = stmt.query_map(rusqlite::params![slug], |row| hydrate_row(conn, row))?;
    match rows.next() {
        Some(Ok(entry)) => Ok(Some(entry)),
        _ => Ok(None),
    }
}

pub fn get_content_paginated(
    conn: &Connection,
    page: u64,
    per_page: u64,
) -> SqlResult<Vec<ContentEntry>> {
    let offset = page.saturating_sub(1) * per_page;
    let mut stmt = conn.prepare(&format!(
        "{SELECT_CONTENT} ORDER BY date_added DESC, id DESC LIMIT ?1 OFFSET ?2"
    ))?;
    stmt.query_map(rusqlite::params![per_page, offset], |row| {
        hydrate_row(conn, row)
    })?
    .collect()
}

pub fn count_all_content(conn: &Connection) -> SqlResult<u64> {
    conn.query_row("SELECT COUNT(*) FROM content", [], |row| row.get(0))
}

pub fn get_all_content(conn: &Connection) -> SqlResult<Vec<ContentEntry>> {
    let mut stmt = conn.prepare(&format!(
        "{SELECT_CONTENT} ORDER BY date_added DESC, id DESC"
    ))?;
    stmt.query_map([], |row| hydrate_row(conn, row))?.collect()
}

pub fn search_content(conn: &Connection, query: &str) -> SqlResult<Vec<ContentEntry>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(&format!(
        "{SELECT_CONTENT} WHERE title LIKE ?1 OR body_markdown LIKE ?1 OR id IN (
            SELECT content_id FROM content_tags WHERE tag LIKE ?1
        ) ORDER BY date_added DESC, id DESC"
    ))?;
    stmt.query_map(rusqlite::params![pattern], |row| hydrate_row(conn, row))?
        .collect()
}

pub fn get_content_by_tag(conn: &Connection, tag: &str) -> SqlResult<Vec<ContentEntry>> {
    let mut stmt = conn.prepare(&format!(
        "{SELECT_CONTENT} WHERE id IN (
            SELECT content_id FROM content_tags WHERE tag = ?1
        ) ORDER BY date_added DESC, id DESC"
    ))?;
    stmt.query_map(rusqlite::params![tag], |row| hydrate_row(conn, row))?
        .collect()
}

pub fn get_content_by_category(conn: &Connection, category: &str) -> SqlResult<Vec<ContentEntry>> {
    let mut stmt = conn.prepare(&format!(
        "{SELECT_CONTENT} WHERE category = ?1 ORDER BY date_added DESC, id DESC"
    ))?;
    stmt.query_map(rusqlite::params![category], |row| hydrate_row(conn, row))?
        .collect()
}

#[cfg(test)]
pub fn get_dynamic_page(conn: &Connection, path: &str) -> SqlResult<Option<DynamicPage>> {
    let mut stmt =
        conn.prepare("SELECT path, title, content, keywords FROM dynamic_pages WHERE path = ?1")?;
    let mut rows = stmt.query_map(rusqlite::params![path], |row| {
        let kw_str: String = row.get(3)?;
        Ok(DynamicPage {
            path: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            keywords: kw_str.split(',').map(|s| s.to_string()).collect(),
        })
    })?;
    match rows.next() {
        Some(Ok(page)) => Ok(Some(page)),
        _ => Ok(None),
    }
}

#[cfg(test)]
pub fn get_all_tags(conn: &Connection) -> SqlResult<Vec<(String, u64)>> {
    let mut stmt =
        conn.prepare("SELECT tag, COUNT(*) as count FROM content_tags GROUP BY tag ORDER BY tag")?;
    stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect()
}

#[cfg(test)]
pub fn get_all_categories(conn: &Connection) -> SqlResult<Vec<(String, u64)>> {
    let mut stmt = conn.prepare(
        "SELECT category, COUNT(*) as count FROM content GROUP BY category ORDER BY category",
    )?;
    stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect()
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = init_db(":memory:").unwrap();
        let entries = vec![
            ContentEntry {
                id: 0,
                slug: "goblin-lore".into(),
                title: "Goblin Lore".into(),
                body_markdown: "Goblin lore content".into(),
                body_html: "<p>Goblin lore content</p>".into(),
                category: "lore".into(),
                tags: vec!["goblin".into(), "lore".into()],
                references: vec![],
                sources: vec![],
                is_dynamic: false,
                date_added: "2026-01-01T00:00:00Z".into(),
                image: None,
            },
            ContentEntry {
                id: 0,
                slug: "goblin-tricks".into(),
                title: "Goblin Tricks".into(),
                body_markdown: "Goblin tricks content".into(),
                body_html: "<p>Goblin tricks content</p>".into(),
                category: "tricks".into(),
                tags: vec!["goblin".into(), "tricks".into()],
                references: vec![],
                sources: vec![],
                is_dynamic: false,
                date_added: "2026-01-02T00:00:00Z".into(),
                image: None,
            },
            ContentEntry {
                id: 0,
                slug: "sam-altman-goblins".into(),
                title: "Sam Altman Goblins".into(),
                body_markdown: "Sam Altman goblin content".into(),
                body_html: "<p>Sam Altman goblin content</p>".into(),
                category: "pop_culture".into(),
                tags: vec!["sam-altman".into(), "ai".into()],
                references: vec![],
                sources: vec![],
                is_dynamic: false,
                date_added: "2026-01-03T00:00:00Z".into(),
                image: None,
            },
        ];

        for entry in &entries {
            insert_content(&conn, entry).unwrap();
        }
        conn
    }

    #[test]
    fn get_content_by_tag_returns_matching_articles() {
        let conn = setup_test_db();
        let results = get_content_by_tag(&conn, "goblin").unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|e| e.slug == "goblin-lore"));
        assert!(results.iter().any(|e| e.slug == "goblin-tricks"));
    }

    #[test]
    fn get_content_by_tag_returns_none_for_unused_tag() {
        let conn = setup_test_db();
        assert!(get_content_by_tag(&conn, "nonexistent").unwrap().is_empty());
    }

    #[test]
    fn get_content_by_category_returns_matching_articles() {
        let conn = setup_test_db();
        let results = get_content_by_category(&conn, "lore").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "goblin-lore");
    }

    #[test]
    fn get_all_tags_returns_counts() {
        let conn = setup_test_db();
        let tags = get_all_tags(&conn).unwrap();
        assert_eq!(tags.len(), 5);
        let goblin_count = tags
            .iter()
            .find(|(t, _)| t == "goblin")
            .map(|(_, c)| *c)
            .unwrap();
        assert_eq!(goblin_count, 2);
    }

    #[test]
    fn get_all_categories_returns_counts() {
        let conn = setup_test_db();
        let cats = get_all_categories(&conn).unwrap();
        assert_eq!(cats.len(), 3);
    }

    #[test]
    fn search_content_matches_title() {
        let conn = setup_test_db();
        let results = search_content(&conn, "Sam Altman").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "sam-altman-goblins");
    }

    #[test]
    fn search_content_matches_tag() {
        let conn = setup_test_db();
        let results = search_content(&conn, "tricks").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "goblin-tricks");
    }

    #[test]
    fn search_content_empty_query_returns_all() {
        let conn = setup_test_db();
        assert_eq!(search_content(&conn, "").unwrap().len(), 3);
    }

    #[test]
    fn count_all_content_works() {
        let conn = setup_test_db();
        assert_eq!(count_all_content(&conn).unwrap(), 3);
    }

    #[test]
    fn get_content_by_slug_found() {
        let conn = setup_test_db();
        let result = get_content_by_slug(&conn, "goblin-lore").unwrap();
        assert_eq!(result.unwrap().title, "Goblin Lore");
    }

    #[test]
    fn get_content_by_slug_not_found() {
        let conn = setup_test_db();
        assert!(get_content_by_slug(&conn, "nonexistent").unwrap().is_none());
    }

    #[test]
    fn dynamic_page_query_compiles() {
        let conn = setup_test_db();
        assert!(get_dynamic_page(&conn, "/missing").unwrap().is_none());
    }
}
