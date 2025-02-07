mod test_post_path_{
    use axum_test::TestServer;

    use crate::tests::setup_server;

    #[tokio::test]
    async fn tests() {
        let server = setup_server().await;

        success(server).await;
    }

    async fn success(server: &TestServer) {
        let response = server.post("/path/name").await;

        response.assert_text("Hello name");
    }
}
