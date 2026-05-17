use axum::extract::{Path, State};

use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::infra::db;

pub async fn raw_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;
    match db::get_content_by_slug(&conn, &slug)? {
        Some(entry) => Ok(entry.body_markdown),
        None => Err(AppError::NotFound(format!("No content found for: {slug}"))),
    }
}
