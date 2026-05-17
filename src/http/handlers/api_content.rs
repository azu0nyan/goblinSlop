use axum::Json;
use axum::extract::{Path, State};

use crate::domain::ContentEntry;
use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::response::ApiResponse;
use crate::infra::db;

pub async fn api_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<ApiResponse<Option<ContentEntry>>>> {
    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;
    let entry = db::get_content_by_slug(&conn, &slug)?;
    Ok(Json(ApiResponse {
        success: entry.is_some(),
        data: entry,
    }))
}
