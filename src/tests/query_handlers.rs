#[cfg(test)]
mod test_post_query {
    use std::sync::LazyLock;

    use axum::{http::StatusCode, Router};
    use axum_test::TestServer;

    use crate::routes::configure_routes;

    static APP: LazyLock<Router> = LazyLock::new(configure_routes);

    #[tokio::test]
    async fn error_without_name() {
        let server = TestServer::builder().build((*APP).clone()).unwrap();

        let response = server.post("/query").await;

        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn success() {
        let server = TestServer::builder().build((*APP).clone()).unwrap();

        let response = server.post("/query?name=Test").await;

        response.assert_status(StatusCode::OK);
        response.assert_text("Hello Test");
    }
}