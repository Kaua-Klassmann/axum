mod test_post_path_{
    use crate::tests::setup_server;

    #[tokio::test]
    async fn success() {
        let server = setup_server().await;

        let response = server.post("/path/name").await;

        response.assert_text("Hello name");
    }
}
