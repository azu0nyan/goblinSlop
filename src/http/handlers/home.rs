use axum::extract::{Query, State};
use axum::response::Html;
use serde::Deserialize;

use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::http::view::{render_card_grid, render_static_page};
use crate::infra::db;

const PER_PAGE: u64 = 12;

#[derive(Deserialize, Default)]
pub struct HomeQuery {
    pub page: Option<u64>,
}

pub async fn home_page(
    State(state): State<AppState>,
    Query(params): Query<HomeQuery>,
) -> AppResult<Html<String>> {
    let conn = state.db.lock().map_err(|_| poisoned())?;
    let current_page = params.page.unwrap_or(1).max(1);

    let entries = db::get_content_paginated(&conn, current_page, PER_PAGE)?;
    let total = db::count_all_content(&conn)?;
    let total_pages = total.div_ceil(PER_PAGE);

    let cards_html = render_card_grid(&entries, true);

    let mut pagination_html = String::new();
    if total_pages > 1 {
        pagination_html.push_str(&format!(
            "<nav class='pagination'><span class='pagination-info'>Page {current_page} of {total_pages}</span>"
        ));

        if current_page > 1 {
            let prev = current_page - 1;
            pagination_html.push_str(&format!(
                "<a href='/?page={prev}' class='pagination-link'>&laquo; Previous</a>"
            ));
        }

        let start_page = if current_page > 2 {
            current_page - 2
        } else {
            1
        };
        let end_page = std::cmp::min(start_page + 4, total_pages);

        for p in start_page..=end_page {
            if p == current_page {
                pagination_html.push_str(&format!("<span class='pagination-current'>{p}</span>"));
            } else {
                pagination_html.push_str(&format!(
                    "<a href='/?page={p}' class='pagination-link'>{p}</a>"
                ));
            }
        }

        if current_page < total_pages {
            let next = current_page + 1;
            pagination_html.push_str(&format!(
                "<a href='/?page={next}' class='pagination-link'>Next &raquo;</a>"
            ));
        }
        pagination_html.push_str("</nav>");
    }

    let body = format!(
        r#"<section class='hero'>
            <h2>🧌 Welcome to the Goblin Realm</h2>
            <p>A collection of goblin-related knowledge, folklore, and cultural references — including the curious connection between Sam Altman, schizophrenia, and goblin trickery.</p>
            <p>Every URL leads somewhere goblin.</p>
        </section>
        <div class='section-header'>
            <h2>Available Content</h2>
            <span class='count-badge'>{total} article{plural}</span>
        </div>
        {cards_html}
        {pagination_html}"#,
        plural = if total == 1 { "" } else { "s" },
    );

    Ok(Html(render_static_page(
        "GoblinSlop — A Library of Goblin Lore",
        &body,
        "home",
        "goblins,home,welcome",
        "/",
        &state.base_url,
    )))
}

fn poisoned() -> AppError {
    AppError::Internal(anyhow::anyhow!("db mutex poisoned"))
}
