#[cfg(test)]
pub mod setup_server {
    use axum_test::TestServer;

    use crate::routes::configure_routes;

    pub fn setup_server() -> TestServer {
        TestServer::builder()
            .build(configure_routes())
            .unwrap()
    }
}