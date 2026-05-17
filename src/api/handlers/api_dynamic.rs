use axum::Json;
use axum::extract::{Path, State};

use crate::api::AppState;
use crate::api::response::ApiResponse;
use crate::domain::DynamicPage;
use crate::domain::services::PageContent;
use crate::error::AppResult;

pub async fn api_dynamic(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> AppResult<Json<ApiResponse<DynamicPage>>> {
    let page = match state.content.resolve_path(&path)? {
        PageContent::Dynamic(p) => p,
        PageContent::Static(entry) => DynamicPage {
            path: path.clone(),
            title: entry.title,
            content: entry.body_html,
            keywords: entry.tags,
        },
    };
    Ok(Json(ApiResponse {
        success: true,
        data: page,
    }))
}
