use axum::extract::State;
use axum::http::header;

use crate::error::{AppError, AppResult};
use crate::http::AppState;
use crate::infra::db;

pub async fn sitemap(
    State(state): State<AppState>,
) -> AppResult<([(header::HeaderName, &'static str); 1], String)> {
    let conn = state
        .db
        .lock()
        .map_err(|_| AppError::Internal(anyhow::anyhow!("db mutex poisoned")))?;
    let entries = db::get_all_content(&conn)?;
    let base_url = state.base_url.trim_end_matches('/').to_string();

    let mut urls = String::new();
    urls.push_str(&format!(
        r#"<url><loc>{base_url}/</loc><changefreq>daily</changefreq><priority>1.0</priority></url>"#,
    ));
    urls.push_str(&format!(
        r#"<url><loc>{base_url}/search</loc><changefreq>weekly</changefreq><priority>0.5</priority></url>"#,
    ));
    for entry in &entries {
        urls.push_str(&format!(
            r#"<url><loc>{base_url}/{}</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>"#,
            entry.slug
        ));
    }

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{urls}
</urlset>"#
    );

    Ok(([(header::CONTENT_TYPE, "application/xml")], xml))
}
