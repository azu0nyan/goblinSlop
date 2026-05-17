use anyhow::Result;

use crate::domain::ContentEntry;

/// Read-only port over the content store.
///
/// Implementations live in `infra::repositories`. Domain code and services depend
/// on this trait, never on a concrete database. Methods are synchronous because
/// the only current implementation wraps a sync rusqlite connection; callers
/// invoke them from within tokio tasks and accept the blocking call.
pub trait ContentRepository: Send + Sync {
    fn get_by_slug(&self, slug: &str) -> Result<Option<ContentEntry>>;
    fn get_paginated(&self, page: u64, per_page: u64) -> Result<Vec<ContentEntry>>;
    fn count_all(&self) -> Result<u64>;
    fn get_all(&self) -> Result<Vec<ContentEntry>>;
    fn search(&self, query: &str) -> Result<Vec<ContentEntry>>;
    fn get_by_tag(&self, tag: &str) -> Result<Vec<ContentEntry>>;
    fn get_by_category(&self, category: &str) -> Result<Vec<ContentEntry>>;
}
