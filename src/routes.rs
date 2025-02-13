use axum::{
    Router,
    routing::{get, post, put}
};

use crate::{handlers, state::AppState};

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .merge(default_routes())
        .nest("/user", user_routes())
        .nest("/post", post_routes())
}

fn default_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/path/{name}", post(handlers::path::path))
        .route("/query", post(handlers::query::query))
        .route("/json", post(handlers::json::json))
        .route("/token", get(handlers::token::token))
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::user::create_user))
        .route("/login", post(handlers::user::login))
        .route("/update/name", put(handlers::user::update_name))
}

fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handlers::post::create_post))
}