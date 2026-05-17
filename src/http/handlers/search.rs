use axum::extract::{Query, State};
use axum::response::Html;
use serde::Deserialize;

use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::view::{render_card_grid, render_static_page};
use crate::infra::db;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn search_page(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> AppResult<Html<String>> {
    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;

    let body = if let Some(query) = &params.q {
        let results = db::search_content(&conn, query)?;
        let header = format!(
            "<p>Search results for <strong>{query}</strong>: {} found.</p>",
            results.len()
        );
        format!("{header}{}", render_card_grid(&results, false))
    } else {
        r#"<form action='/search' method='GET' class='search-form'>
            <input type='text' name='q' placeholder='Search goblin knowledge...'>
            <button type='submit'>🔍 Search</button>
        </form>
        <p>Try searching for: <a href='/search?q=goblin'>goblin</a>, <a href='/search?q=sam'>sam</a>, <a href='/search?q=trick'>trick</a>, <a href='/search?q=schizophrenia'>schizophrenia</a></p>
        <p>Or explore any hidden goblin path!</p>"#.to_string()
    };

    Ok(Html(render_static_page(
        "Search GoblinSlop",
        &body,
        "search",
        "search",
        "/search",
        &state.base_url,
    )))
}
