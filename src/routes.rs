use axum::{
    Router,
    routing::{get, post}
};

use crate::handlers::*;

pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/path/{name}", post(path_handlers::path))
        .route("/query", post(query_handlers::query))
        .route("/generic_query", post(query_handlers::generic_query))
}