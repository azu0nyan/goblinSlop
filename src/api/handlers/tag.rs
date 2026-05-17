use axum::extract::{Path, State};
use axum::response::Html;

use crate::api::AppState;
use crate::api::view::{render_card_grid, render_static_page};
use crate::error::AppResult;

pub async fn tag_page(
    State(state): State<AppState>,
    Path(tag): Path<String>,
) -> AppResult<Html<String>> {
    let entries = state.content.by_tag(&tag)?;
    let count = entries.len();

    let mut body = format!("<h2>Articles tagged: <span class='tag-link'>{tag}</span></h2>");
    if count == 0 {
        body.push_str("<p>No articles found with this tag.</p>");
    } else {
        body.push_str(&format!(
            "<p>{count} article{} found.</p>",
            if count == 1 { "" } else { "s" }
        ));
        body.push_str(&render_card_grid(&entries, true));
    }

    Ok(Html(render_static_page(
        &format!("Tag: {tag} - GoblinSlop"),
        &body,
        "tag",
        &tag,
        &format!("/tag/{tag}"),
        &state.base_url,
    )))
}
