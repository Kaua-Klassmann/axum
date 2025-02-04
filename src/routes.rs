use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post}
};

use crate::config::cors::get_cors;
use crate::handlers::*;
use crate::state::get_state;

pub async fn configure_routes() -> Router {
    let state = Arc::new(
        get_state().await
    );

    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/path/{name}", post(path_handlers::path))
        .route("/query", post(query_handlers::query))
        .route("/json", post(json_handlers::json))
        .route("/create_user", post(user_handlers::create_user))
        .layer(get_cors())
        .with_state(state)
}
