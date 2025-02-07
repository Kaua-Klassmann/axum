mod test_post_query {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    use crate::tests::setup_server;

    #[tokio::test]
    async fn tests() {
        let server = setup_server().await;

        error_without_name(server).await;
        success(server).await
    }

    async fn error_without_name(server: &TestServer) {
        let response = server.post("/query").await;

        response.assert_status(StatusCode::BAD_REQUEST);
    }

    async fn success(server: &TestServer) {
        let response = server.post("/query?name=Test").await;

        response.assert_status(StatusCode::OK);
        response.assert_text("Hello Test");
    }
}
