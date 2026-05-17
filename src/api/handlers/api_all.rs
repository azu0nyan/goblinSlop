use axum::Json;
use axum::extract::State;

use crate::api::AppState;
use crate::api::response::ApiResponse;
use crate::domain::ContentEntry;
use crate::error::AppResult;

pub async fn api_all(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Vec<ContentEntry>>>> {
    let entries = state.content.all()?;
    Ok(Json(ApiResponse {
        success: true,
        data: entries,
    }))
}
