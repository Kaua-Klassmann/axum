use std::{collections::HashMap, env};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{jwt::JwtClaims, state::AppState};

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

pub async fn chat(
    _: JwtClaims,
    State(state): State<AppState>,
    Json(payload): Json<ChatPayload>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Schema invalid"
            })),
        );
    }

    let client = &state.reqwest_client;

    let gemini_api = env::var("GEMINI_API").expect("GEMINI_API not found at .env file");

    let json_payload = json!({
        "contents": [{
            "parts": [{
                "text": payload.text
            }]
        }]
    });

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
