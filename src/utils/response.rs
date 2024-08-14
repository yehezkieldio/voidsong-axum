use axum::http::{header, HeaderName, StatusCode};
use axum::response::{IntoResponse, Response};

use crate::env::VERSION;

pub enum VoidsongError {
    FailedToFetchImage,
    ServiceUnavailable,
}

impl IntoResponse for VoidsongError {
    fn into_response(self) -> Response {
        let headers = [
            (header::CACHE_CONTROL, "no-cache"),
            (HeaderName::from_static("x-voidsong-version"), VERSION),
        ];

        let body = match self {
            VoidsongError::FailedToFetchImage => "failed to fetch image data",
            VoidsongError::ServiceUnavailable => "service unavailable",
        };

        (StatusCode::SERVICE_UNAVAILABLE, headers, body).into_response()
    }
}

pub struct VoidsongImage {
    pub content_type: String,
    pub bytes: Vec<u8>,
}

impl IntoResponse for VoidsongImage {
    fn into_response(self) -> Response {
        let headers = [
            (header::CONTENT_TYPE, self.content_type.as_str()),
            (header::CACHE_CONTROL, "no-cache"),
            (HeaderName::from_static("x-voidsong-version"), VERSION),
        ];

        (StatusCode::OK, headers, self.bytes).into_response()
    }
}
