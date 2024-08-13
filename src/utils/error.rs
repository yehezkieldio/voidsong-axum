use axum::response::IntoResponse;
use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VoidsongError {
    message: String,
}

pub fn json_error(message: &str) -> impl IntoResponse {
    let error = VoidsongError {
        message: message.to_string(),
    };

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&error).unwrap(),
    )
        .into_response()
}
