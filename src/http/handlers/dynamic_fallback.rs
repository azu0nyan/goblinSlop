use axum::extract::State;
use axum::http::Request;
use axum::response::{Html, IntoResponse, Redirect, Response};
use rand::thread_rng;

use crate::domain::generator::{generate_dynamic_page_content, parse_path_into_keywords};
use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::view::{render_content_page, render_dynamic_page};
use crate::infra::db;

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

    // Normalize underscore variants to the canonical hyphen form (301).
    if slug.contains('_') {
        return Ok(Redirect::permanent(&format!("/{}", normalize_slug(&slug))).into_response());
    }

    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;

    if let Some(entry) = db::get_content_by_slug(&conn, &slug)? {
        return Ok(Html(render_content_page(
            &entry,
            &format!("/{slug}"),
            &state.base_url,
        ))
        .into_response());
    }
    drop(conn);

    let keywords = parse_path_into_keywords(&slug);
    let final_keywords = if keywords.is_empty() {
        vec!["goblin".to_string(), "mystery".to_string(), slug.clone()]
    } else {
        keywords
    };

    let dyn_page = generate_dynamic_page_content(&slug, &final_keywords);
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
