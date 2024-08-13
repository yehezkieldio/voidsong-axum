use axum::response::IntoResponse;
use reqwest::{header, Client, Error, Response as ReqwestResponse, StatusCode};
use serde::Deserialize;

use crate::VoidsongError;

pub async fn get_random_cat() -> Result<impl IntoResponse, impl IntoResponse> {
    let client: Client = Client::new();
    let url: &str = "https://cataas.com/cat";

    let result: Result<ReqwestResponse, Error> = client.get(url).send().await;

    match result {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                Ok((
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "image/jpeg")],
                    res.bytes().await.unwrap(),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to fetch random cat image",
                )
                    .into_response())
            }
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch random cat image",
        )
            .into_response()),
    }
}

#[derive(Deserialize)]
struct DogResponse {
    message: String,
}

pub async fn get_random_dog() -> Result<impl IntoResponse, impl IntoResponse> {
    let client: Client = Client::new();
    let url: &str = "https://dog.ceo/api/breeds/image/randoms";

    let result: Result<ReqwestResponse, Error> = client.get(url).send().await;
    let data: DogResponse = result.unwrap().json::<DogResponse>().await.unwrap();

    let result: Result<ReqwestResponse, Error> = client.get(&data.message).send().await;

    let error: VoidsongError = VoidsongError {
        message: "Failed to fetch random cat image".to_string(),
    };

    match result {
        Ok(res) => {
            if res.status() == StatusCode::OK {
                Ok((
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "image/jpeg")],
                    res.bytes().await.unwrap(),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(header::CONTENT_TYPE, "application/json")],
                    serde_json::to_string(&error).unwrap(),
                )
                    .into_response())
            }
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json")],
            serde_json::to_string(&error).unwrap(),
        )
            .into_response()),
    }
}
