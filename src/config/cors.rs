use std::env;

use axum::http::HeaderValue;
use tokio::sync::OnceCell;
use tower_http::cors::{Any, CorsLayer};

static CORS: OnceCell<CorsLayer> = OnceCell::const_new();

pub async fn get_cors() -> &'static CorsLayer {
    CORS.get_or_init(configure_cors).await
}

async fn configure_cors() -> CorsLayer {
    let origin = env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN not found at .env file")
        .parse::<HeaderValue>()
        .unwrap();

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(Any)
        .allow_headers(Any)
}