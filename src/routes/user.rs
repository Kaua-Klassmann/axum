use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{handlers, state::AppState};

pub fn get_user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::user::create_user))
        .route(
            "/activate/{validation_uuid}",
            get(handlers::user::active_user),
        )
        .route("/login", post(handlers::user::login))
        .route("/update/name", put(handlers::user::update_name))
}
