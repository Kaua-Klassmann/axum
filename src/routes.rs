use axum::{
    Router,
    routing::{get, post}
};

use crate::{handlers, state::AppState};

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/path/{name}", post(handlers::path::path))
        .route("/query", post(handlers::query::query))
        .route("/json", post(handlers::json::json))
        .route("/create_user", post(handlers::user::create_user))
        .route("/login", post(handlers::user::login))
}
