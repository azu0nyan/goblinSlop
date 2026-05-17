use axum::Json;
use axum::extract::{Path, State};

use crate::api::AppState;
use crate::api::response::ApiResponse;
use crate::domain::ContentEntry;
use crate::error::AppResult;

pub async fn api_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> AppResult<Json<ApiResponse<Option<ContentEntry>>>> {
    let entry = state.content.find_by_slug(&slug)?;
    Ok(Json(ApiResponse {
        success: entry.is_some(),
        data: entry,
    }))
}
