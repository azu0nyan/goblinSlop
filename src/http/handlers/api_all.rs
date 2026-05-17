use axum::Json;
use axum::extract::State;

use crate::domain::ContentEntry;
use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::response::ApiResponse;
use crate::infra::db;

pub async fn api_all(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Vec<ContentEntry>>>> {
    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;
    let entries = db::get_all_content(&conn)?;
    Ok(Json(ApiResponse {
        success: true,
        data: entries,
    }))
}
