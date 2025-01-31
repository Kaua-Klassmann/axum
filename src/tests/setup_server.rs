use axum_test::TestServer;
use dotenvy::dotenv;

use crate::routes::configure_routes;

pub async fn setup_server() -> TestServer {
    dotenv().ok();

    TestServer::builder()
        .build(configure_routes().await)
        .unwrap()
}
