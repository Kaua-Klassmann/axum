use std::sync::Arc;

use argon2::Argon2;
use axum_test::TestServer;
use dotenvy::dotenv;
use tokio::sync::OnceCell;

use crate::{
    database::database::get_db_connection,
    routes::configure_routes,
    state::AppState
};

mod path_handlers;
mod query_handlers;
mod json_handlers;

static SERVER: OnceCell<TestServer> = OnceCell::const_new();

pub async fn setup_server() -> &'static TestServer {
    SERVER.get_or_init(|| async {
        dotenv().ok();

        let db_conn = get_db_connection().await;
        let argon2 = Arc::new(Argon2::default());

        let state = AppState {
            db_conn,
            argon2
        };

        TestServer::builder()
            .build(
                configure_routes()
                    .with_state(state)
            )
            .unwrap()
    })
    .await
}

