use std::{collections::HashMap, env};

use axum::{http::StatusCode, response::IntoResponse, Json};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::jwt::JwtClaims;

#[derive(Deserialize, Validate)]
pub struct ChatPayload {
    #[validate(length(min = 1))]
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

pub async fn chat(_: JwtClaims, Json(payload): Json<ChatPayload>) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Schema invalid"
            })),
        );
    }

    let gemini_api = env::var("GEMINI_API").expect("GEMINI_API not found at .env file");

    let mut text = HashMap::new();
    text.insert("text", payload.text);

    let mut parts = HashMap::new();
    parts.insert("parts", [text]);

    let mut json_payload = HashMap::new();
    json_payload.insert("contents", [parts]);

    let client = Client::new();

    let res_result = client.post(gemini_api).json(&json_payload).send().await;

    if res_result.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to connect on ChatBot"
            })),
        );
    }

    let res: GeminiResponse = res_result.unwrap().json().await.unwrap();
    let text = res.candidates[0].content.parts[0]
        .text
        .clone()
        .trim()
        .to_string();

    return (
        StatusCode::OK,
        Json(json!({
            "text": text
        })),
    );
}
