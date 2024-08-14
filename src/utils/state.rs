use std::sync::Arc;

use reqwest::Client;
use tokio::sync::Mutex;

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
