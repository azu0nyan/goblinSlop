use axum::extract::{Path, State};
use axum::response::Html;

use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::view::{render_card_grid, render_static_page};
use crate::infra::db;

pub async fn category_page(
    State(state): State<AppState>,
    Path(category): Path<String>,
) -> AppResult<Html<String>> {
    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;
    let entries = db::get_content_by_category(&conn, &category)?;
    let count = entries.len();

    let mut body = format!("<h2>Category: <span class='category-link'>{category}</span></h2>");
    if count == 0 {
        body.push_str("<p>No articles found in this category.</p>");
    } else {
        body.push_str(&format!(
            "<p>{count} article{} found.</p>",
            if count == 1 { "" } else { "s" }
        ));
        body.push_str(&render_card_grid(&entries, true));
    }

    Ok(Html(render_static_page(
        &format!("Category: {category} - GoblinSlop"),
        &body,
        "category",
        &category,
        &format!("/category/{category}"),
        &state.base_url,
    )))
}
