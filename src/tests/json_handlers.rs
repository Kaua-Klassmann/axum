mod test_post_json {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde::{Deserialize, Serialize};

    use crate::tests::setup_server;

    #[tokio::test]
    async fn tests() {
        let server = setup_server().await;

        error_without_json(server).await;
        error_without_name(server).await;
        has_2_or_less_letters(server).await;
        has_3_or_more_letters(server).await;
    }

    async fn error_without_json(server: &TestServer) {
        let response = server.post("/json").await;

        response.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    async fn error_without_name(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload;

        let response = server.post("/json")
            .json(&RequestPayload{})
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    async fn has_2_or_less_letters(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            name: String,
            has_3_or_more_letters: bool
        }

        let response = server.post("/json")
            .json(&RequestPayload {
                name: "RS".to_string()
            })
            .await;

        response.assert_status(StatusCode::OK);
        response.assert_json(&ResponsePayload {
            name: "RS".to_string(),
            has_3_or_more_letters: false
        });
    }

    async fn has_3_or_more_letters(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            name: String,
            has_3_or_more_letters: bool
        }

        let response = server.post("/json")
            .json(&RequestPayload {
                name: "Test".to_string()
            })
            .await;

        response.assert_status(StatusCode::OK);
        response.assert_json(&ResponsePayload {
            name: "Test".to_string(),
            has_3_or_more_letters: true
        });
    }
}
