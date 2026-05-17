use std::sync::{Arc, Mutex};

use anyhow::{Context, Result, anyhow};
use rusqlite::Connection;

use crate::domain::ContentEntry;
use crate::domain::ports::ContentRepository;
use crate::infra::db;

/// SQLite-backed implementation of `ContentRepository`. Wraps an
/// `Arc<Mutex<Connection>>`; cloning the repo is cheap because cloning the
/// `Arc` is. The mutex is held only for the duration of a single query.
#[derive(Clone)]
pub struct SqliteContentRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteContentRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|_| anyhow!("db mutex poisoned"))
    }
}

impl ContentRepository for SqliteContentRepo {
    fn get_by_slug(&self, slug: &str) -> Result<Option<ContentEntry>> {
        let conn = self.lock()?;
        db::get_content_by_slug(&conn, slug).with_context(|| format!("get_by_slug({slug})"))
    }

    fn get_paginated(&self, page: u64, per_page: u64) -> Result<Vec<ContentEntry>> {
        let conn = self.lock()?;
        db::get_content_paginated(&conn, page, per_page).context("get_paginated")
    }

    fn count_all(&self) -> Result<u64> {
        let conn = self.lock()?;
        db::count_all_content(&conn).context("count_all")
    }

    fn get_all(&self) -> Result<Vec<ContentEntry>> {
        let conn = self.lock()?;
        db::get_all_content(&conn).context("get_all")
    }

    fn search(&self, query: &str) -> Result<Vec<ContentEntry>> {
        let conn = self.lock()?;
        db::search_content(&conn, query).with_context(|| format!("search({query})"))
    }

    fn get_by_tag(&self, tag: &str) -> Result<Vec<ContentEntry>> {
        let conn = self.lock()?;
        db::get_content_by_tag(&conn, tag).with_context(|| format!("get_by_tag({tag})"))
    }

    fn get_by_category(&self, category: &str) -> Result<Vec<ContentEntry>> {
        let conn = self.lock()?;
        db::get_content_by_category(&conn, category)
            .with_context(|| format!("get_by_category({category})"))
    }
}
