use axum::{
    routing::{get, post}, Router
};

use crate::handlers::*;

pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(static_handlers::hello))
        .route("/path/:name", get(path_handlers::path))
        .route("/query", get(query_handlers::query))
        .route("/json", post(json_handlers::json))
}