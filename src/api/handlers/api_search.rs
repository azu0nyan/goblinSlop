use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;

use crate::api::AppState;
use crate::api::response::ApiResponse;
use crate::domain::ContentEntry;
use crate::error::AppResult;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn api_search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> AppResult<Json<ApiResponse<Vec<ContentEntry>>>> {
    let results = match &params.q {
        Some(q) => state.content.search(q)?,
        None => state.content.all()?,
    };
    Ok(Json(ApiResponse {
        success: true,
        data: results,
    }))
}
