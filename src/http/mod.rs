pub mod handlers;
pub mod response;
pub mod view;

use axum::Router;
use axum::routing::get;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub base_url: String,
    /// Cached image pool, scanned once at startup.
    pub image_pool: Arc<Vec<String>>,
}

pub fn create_router(state: AppState) -> Router {
    use handlers::*;

    let app = Router::new()
        .route("/", get(home::home_page))
        .route("/search", get(search::search_page))
        .route("/raw/:slug", get(raw::raw_content))
        .route("/tag/:tag", get(tag::tag_page))
        .route("/category/:category", get(category::category_page))
        .route("/api/content/:slug", get(api_content::api_content))
        .route("/api/dynamic/*path", get(api_dynamic::api_dynamic))
        .route("/api/search", get(api_search::api_search))
        .route("/sitemap.xml", get(sitemap::sitemap))
        .route("/api/all", get(api_all::api_all))
        .with_state(state.clone());

    Router::new()
        .fallback(get(dynamic_fallback::dynamic_fallback))
        .with_state(state)
        .merge(app)
}
