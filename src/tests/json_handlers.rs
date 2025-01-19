#[cfg(test)]
mod test_post_json {
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};

    use crate::tests::setup_server::setup_server::setup_server;

    #[tokio::test]
    async fn error_without_json() {
        let server = setup_server();

        let response = server.post("/json").await;

        response.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn error_without_name() {
        #[derive(Serialize)]
        struct RequestPayload;

        let server = setup_server();

        let response = server.post("/json")
            .json(&RequestPayload{})
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn has_2_or_less_letters() {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            name: String,
            has_3_or_more_letters: bool
        }

        let server = setup_server();

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

    #[tokio::test]
    async fn has_3_or_more_letters() {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            name: String,
            has_3_or_more_letters: bool
        }

        let server = setup_server();

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