use axum::extract::State;
use reqwest::{Client, Response};
use serde::Deserialize;
use tokio::sync::MutexGuard;

use crate::utils::{
    response::{VoidsongError, VoidsongImage},
    state::{user_agent, AppState},
    url::{fetch_image, preflight_check},
};

#[derive(Deserialize)]
struct DogResponse {
    message: String,
}

pub async fn get_random_dog(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec!["https://dog.ceo/api/breeds/image/random"];
    let client: MutexGuard<Client> = state.client.lock().await;

    // Check if the APIs are available
    let (success, url) = preflight_check(&client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match client.get(url.unwrap()).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let url: String = match get_url.json::<DogResponse>().await {
        Ok(response) => response.message,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}
