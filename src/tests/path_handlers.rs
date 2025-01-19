#[cfg(test)]
mod test_post_path_{
    use std::sync::LazyLock;

    use axum::Router;
    use axum_test::TestServer; 

    use crate::routes::configure_routes;

    static APP: LazyLock<Router> = LazyLock::new(configure_routes);

    #[tokio::test]
    async fn success() {
        let server = TestServer::builder().build((*APP).clone()).unwrap();

        let response = server.post("/path/name").await;

        response.assert_text("Hello name");
    }
}