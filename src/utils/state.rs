use std::sync::Arc;

use axum::http::HeaderMap;
use reqwest::Client;
use tokio::sync::Mutex;

use crate::env::VERSION;

pub fn user_agent() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        format!("Voidsong/{}", VERSION).parse().unwrap(),
    );

    headers
}

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Mutex<Client>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(Client::new())),
        }
    }
}
