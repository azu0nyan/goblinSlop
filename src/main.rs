use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::EnvFilter;

use goblin_slop::api::{AppState, create_router};
use goblin_slop::config::Config;
use goblin_slop::domain::services::ContentService;
use goblin_slop::infra::repositories::SqliteContentRepo;
use goblin_slop::infra::{content_loader, db, image_pool};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let cfg = Config::from_env();
    cfg.validate().context("invalid configuration")?;
    info!(?cfg, "🧌 GoblinSlop starting");

    // In-memory DB rebuilt from JSON on every boot.
    let conn = db::init_db(":memory:").context("initializing in-memory database")?;
    content_loader::load_all_content_into_conn(&conn, &cfg.content_dir)
        .context("loading content into database")?;

    let repo = SqliteContentRepo::new(Arc::new(Mutex::new(conn)));
    let content = ContentService::new(repo);

    let image_pool = Arc::new(image_pool::scan(format!("{}/images", cfg.static_dir)));
    info!(count = image_pool.len(), "image pool scanned");

    let state = AppState {
        content,
        base_url: cfg.base_url.clone(),
        image_pool,
    };

    let app = create_router(state).nest_service("/static", ServeDir::new(&cfg.static_dir));

    let bind_addr = cfg.bind_addr();
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("binding to {bind_addr}"))?;

    info!(%bind_addr, "🧌 GoblinSlop server running");
    axum::serve(listener, app).await.context("server error")?;

    Ok(())
}
