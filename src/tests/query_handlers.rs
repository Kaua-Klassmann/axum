#[cfg(test)]
mod test_post_query {
    use axum::http::StatusCode;

    use crate::tests::setup_server::setup_server::setup_server;

    #[tokio::test]
    async fn error_without_name() {
        let server = setup_server();

        let response = server.post("/query").await;

        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn success() {
        let server = setup_server();

        let response = server.post("/query?name=Test").await;

        response.assert_status(StatusCode::OK);
        response.assert_text("Hello Test");
    }
}