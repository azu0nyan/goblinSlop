use std::sync::Arc;

use crate::domain::services::ContentService;
use crate::infra::repositories::SqliteContentRepo;

/// Application state shared with every handler. Held in `Arc`-friendly forms so
/// that Axum's per-request clone is cheap.
#[derive(Clone)]
pub struct AppState {
    pub content: ContentService<SqliteContentRepo>,
    pub base_url: String,
    /// Cached image pool, scanned once at startup.
    pub image_pool: Arc<Vec<String>>,
}
