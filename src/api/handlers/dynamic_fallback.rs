use axum::extract::State;
use axum::http::Request;
use axum::response::{Html, IntoResponse, Redirect, Response};
use rand::thread_rng;

use crate::api::AppState;
use crate::api::view::{render_content_page, render_dynamic_page};
use crate::domain::services::PageContent;
use crate::error::AppResult;

fn normalize_slug(slug: &str) -> String {
    slug.replace('_', "-")
}

pub async fn dynamic_fallback(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
) -> AppResult<Response> {
    let uri = req
        .uri()
        .path_and_query()
        .map(|pq| pq.path().to_string())
        .unwrap_or_else(|| "/".to_string());

    let slug = uri.trim_start_matches('/').to_string();
    if slug.is_empty() {
        return Ok(Redirect::permanent("/").into_response());
    }
    if slug.contains('_') {
        return Ok(Redirect::permanent(&format!("/{}", normalize_slug(&slug))).into_response());
    }

    match state.content.resolve_path(&slug)? {
        PageContent::Static(entry) => Ok(Html(render_content_page(
            &entry,
            &format!("/{slug}"),
            &state.base_url,
        ))
        .into_response()),
        PageContent::Dynamic(dyn_page) => {
            let mut rng = thread_rng();
            Ok(Html(render_dynamic_page(
                &dyn_page,
                &format!("/{slug}"),
                &state.base_url,
                &mut rng,
                &state.image_pool,
            ))
            .into_response())
        }
    }
}
