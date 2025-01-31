use axum_test::TestServer;
use dotenvy::dotenv;
use tokio::sync::OnceCell;

use crate::routes::configure_routes;

mod path_handlers;
mod query_handlers;
mod json_handlers;

static SERVER: OnceCell<TestServer> = OnceCell::const_new();

pub async fn setup_server() -> &'static TestServer {
    SERVER.get_or_init(|| async {
        dotenv().ok();

        TestServer::builder()
            .build(configure_routes().await)
            .unwrap()
    })
    .await
}

