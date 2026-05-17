use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;

use crate::domain::ContentEntry;
use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::response::ApiResponse;
use crate::infra::db;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn api_search(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> AppResult<Json<ApiResponse<Vec<ContentEntry>>>> {
    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;
    let results = match &params.q {
        Some(q) => db::search_content(&conn, q)?,
        None => db::get_all_content(&conn)?,
    };
    Ok(Json(ApiResponse {
        success: true,
        data: results,
    }))
}
