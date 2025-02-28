use std::sync::Arc;

use argon2::Argon2;
use axum::Router;
use reqwest::Client as ReqwestClient;

use crate::{
    config::cors::get_cors, database::database::get_db_connection, routes::configure_routes,
    state::AppState,
};

pub async fn create_app() -> Router {
    let db_conn = get_db_connection().await;
    let argon2 = Arc::new(Argon2::default());
    let http_client = Arc::new(ReqwestClient::new());

    let state = AppState {
        db_conn,
        argon2,
        http_client,
    };

    let app = configure_routes().layer(get_cors()).with_state(state);

    app
}
