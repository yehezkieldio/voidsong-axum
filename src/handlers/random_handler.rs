use axum::extract::State;
use reqwest::{Client, Response};
use serde::Deserialize;

use crate::utils::{
    response::{VoidsongError, VoidsongImage},
    state::{user_agent, AppState},
    url::{fetch_image, preflight_check},
};

#[derive(Deserialize)]
struct CatAsService {
    _id: String,
}

#[derive(Deserialize)]
struct TheCatAPI {
    url: String,
}

pub async fn get_random_cat(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec![
        "https://cataas.com/cat?json=true",
        "https://api.thecatapi.com/v1/images/search",
    ];
    let client: Client = state.client;

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

    let url: String = match url.unwrap() {
        "https://cataas.com/cat?json=true" => match get_url.json::<CatAsService>().await {
            Ok(response) => format!("https://cataas.com/cat/{}", response._id),
            Err(_) => return Err(VoidsongError::FailedToFetchImage),
        },
        "https://api.thecatapi.com/v1/images/search" => {
            match get_url.json::<Vec<TheCatAPI>>().await {
                Ok(response) => response[0].url.clone(),
                Err(_) => return Err(VoidsongError::FailedToFetchImage),
            }
        }
        _ => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(&client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}

#[derive(Deserialize)]
struct DogCEO {
    message: String,
}

pub async fn get_random_dog(State(state): State<AppState>) -> Result<VoidsongImage, VoidsongError> {
    let urls: Vec<&str> = vec!["https://dog.ceo/api/breeds/image/random"];
    let client: Client = state.client;

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

    let url: String = match get_url.json::<DogCEO>().await {
        Ok(response) => response.message,
        Err(_) => return Err(VoidsongError::FailedToFetchImage),
    };

    let image: Option<VoidsongImage> = fetch_image(&client, url.as_str()).await;
    image.ok_or(VoidsongError::FailedToFetchImage)
}
