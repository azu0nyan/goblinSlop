use axum::extract::{Path, State};

use crate::api::AppState;
use crate::error::{AppError, AppResult};

pub async fn raw_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<String> {
    match state.content.find_by_slug(&slug)? {
        Some(entry) => Ok(entry.body_markdown),
        None => Err(AppError::NotFound(format!("No content found for: {slug}"))),
    }
}
