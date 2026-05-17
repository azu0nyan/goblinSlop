use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use axum::http::StatusCode;

use crate::db;
use super::super::templates::{render_static_page, render_tags, render_category};
use super::super::AppState;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn search_page(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = state.db.lock().unwrap();

    let body = if let Some(query) = &params.q {
        let results = db::search_content(&db, query).unwrap_or_default();
        let mut list_html = format!("<p>Search results for <strong>{}</strong>: {} found.</p>", query, results.len());
        list_html.push_str("<div class='article-grid'>");
        for entry in &results {
            let date_str = if entry.date_added.len() >= 10 {
                &entry.date_added[..10]
            } else {
                &entry.date_added
            };
            let img_file = entry.image.as_deref().unwrap_or("default.jpg");
            let tag_links = render_tags(&entry.tags);
            let cat_link = render_category(&entry.category);

            list_html.push_str(&format!(
                r#"<div class='article-card'>
                    <div class='card-top'>
                        <div class='card-image'>
                            <img src="/static/images/{}" alt="{}" class="card-img">
                        </div>
                        <div class='card-top-right'>
                            <a href='/{}' class='card-title-link'><h3 class='card-title'>{}</h3></a>
                            <div class='card-header'>
                                <span class='card-date'>{}</span>
                                {}
                            </div>
                        </div>
                    </div>
                    <div class='card-footer'>
                        <span class='card-tags'>{}</span>
                    </div>
                </div>"#,
                img_file,
                entry.title,
                entry.slug,
                entry.title,
                date_str,
                cat_link,
                tag_links,
            ));
        }
        list_html.push_str("</div>");
        list_html
    } else {
        format!(
            r#"<form action='/search' method='GET' class='search-form'>
                <input type='text' name='q' placeholder='Search goblin knowledge...'>
                <button type='submit'>🔍 Search</button>
            </form>
            <p>Try searching for: <a href='/search?q=goblin'>goblin</a>, <a href='/search?q=sam'>sam</a>, <a href='/search?q=trick'>trick</a>, <a href='/search?q=schizophrenia'>schizophrenia</a></p>
            <p>Or explore any hidden goblin path!</p>"#
        )
    };

    Ok(Html(render_static_page(
        "Search GoblinSlop",
        &body,
        "search",
        "search",
        "/search",
        &state.base_url,
        state.use_new_template_engine,
    )))
}