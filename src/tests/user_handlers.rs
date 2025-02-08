mod test_post_create_user {
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde::{Deserialize, Serialize};

    use crate::tests::setup_server;

    #[tokio::test]
    async fn tests() {
        let server = setup_server().await;

        error_without_json(server).await;
        error_without_name(server).await;
        error_without_email(server).await;
        error_without_password(server).await;
        error_name_without_at_least_1_letters(server).await;
        error_email_is_not_email(server).await;
        error_password_without_at_least_6_letters(server).await;
        success(server).await;
        error_user_exist(server).await;
    }

    async fn error_without_json(server: &TestServer) {
        let response = server.post("/create_user").await;

        response.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    async fn error_without_name(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            email: String,
            password: String
        }

        let response = server.post("/create_user")
            .json(&RequestPayload{
                email: "".to_string(),
                password: "".to_string()
            })
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    async fn error_without_email(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String,
            password: String
        }

        let response = server.post("/create_user")
            .json(&RequestPayload{
                name: "".to_string(),
                password: "".to_string()
            })
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    async fn error_without_password(server: &TestServer) {

        #[derive(Serialize)]
        struct RequestPayload {
            name: String,
            email: String
        }

        let response = server.post("/create_user")
            .json(&RequestPayload{
                name: "".to_string(),
                email: "".to_string()
            })
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    async fn error_name_without_at_least_1_letters(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String,
            email: String,
            password: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            error: String
        }

        let response = server.post("/create_user")
            .json(&RequestPayload {
                name: "".to_string(),
                email: "test@gmail.com".to_string(),
                password: "123123".to_string()
            })
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        response.assert_json(&ResponsePayload {
            error: "Schema invalid".to_string()
        });
    }

    async fn error_email_is_not_email(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String,
            email: String,
            password: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            error: String
        }

        let response = server.post("/create_user")
            .json(&RequestPayload {
                name: "test".to_string(),
                email: "test".to_string(),
                password: "123123".to_string()
            })
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        response.assert_json(&ResponsePayload {
            error: "Schema invalid".to_string()
        });
    }

    async fn error_password_without_at_least_6_letters(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String,
            email: String,
            password: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            error: String
        }

        let response = server.post("/create_user")
            .json(&RequestPayload {
                name: "test".to_string(),
                email: "test@gmail.com".to_string(),
                password: "12312".to_string()
            })
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
        response.assert_json(&ResponsePayload {
            error: "Schema invalid".to_string()
        });
    }

    async fn success(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String,
            email: String,
            password: String
        }

        let response = server.post(&"/create_user")
            .json(&RequestPayload {
                name: "test".to_string(),
                email: "test@gmail.com".to_string(),
                password: "123123".to_string()
            })
            .await;

        response.assert_status(StatusCode::OK);
    }

    async fn error_user_exist(server: &TestServer) {
        #[derive(Serialize)]
        struct RequestPayload {
            name: String,
            email: String,
            password: String
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct ResponsePayload {
            error: String
        }

        let response = server.post("/create_user")
            .json(&RequestPayload {
                name: "test".to_string(),
                email: "test@gmail.com".to_string(),
                password: "123123".to_string()
            })
            .await;

        response.assert_status(StatusCode::BAD_REQUEST);
        response.assert_json(&ResponsePayload {
            error: "Failed to create account".to_string()
        });
    }
}