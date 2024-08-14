use axum::{extract::State, response::IntoResponse};
use reqwest::{header, Client, Error, Response as ReqwestResponse, StatusCode};
use serde::Deserialize;
use tokio::sync::MutexGuard;

use crate::utils::{error::json_error, state::AppState};

async fn fetch_image<'a>(
    client: MutexGuard<'a, Client>,
    url: &str,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let response = client.get(url).send().await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let bytes = response.bytes().await;

                match bytes {
                    Ok(bytes) => Ok((
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "image/jpeg")],
                        bytes,
                    )),
                    Err(_) => Err(json_error("Failed to fetch image data.")),
                }
            } else {
                Err(json_error("Failed to fetch image"))
            }
        }
        Err(_) => Err(json_error("Failed to fetch image")),
    }
}

pub async fn get_random_cat(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let url: &str = "https://cataas.com/cat";
    let client = state.client.lock().await;

    fetch_image(client, url).await
}

#[derive(Deserialize)]
struct DogResponse {
    message: String,
}

pub async fn get_random_dog(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let url: &str = "https://dog.ceo/api/breeds/image/random";
    let client = state.client.lock().await;

    let result: Result<ReqwestResponse, Error> = client.get(url).send().await;
    let data: DogResponse = result.unwrap().json::<DogResponse>().await.unwrap();

    fetch_image(client, &data.message).await
}
