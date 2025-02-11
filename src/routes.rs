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
        .nest("/user", user_routes())
        .merge(configure_protected_routes())
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::user::create_user))
        .route("/login", post(handlers::user::login))
}

fn configure_protected_routes() -> Router<AppState> {
    Router::new()
        .route("/token", get(handlers::token::token))
}