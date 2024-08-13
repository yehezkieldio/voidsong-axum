use axum::response::IntoResponse;
use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CatResponse {
    _id: String,
}

pub async fn get_random_cat() -> Result<impl IntoResponse, impl IntoResponse> {
    let client = Client::new();
    let url = "https://cataas.com/cat?json=true";

    let result = client.get(url).send().await;
    let data = result.unwrap().json::<CatResponse>().await.unwrap();

    let result = client
        .get("https://cataas.com/cat/".to_string() + &data._id)
        .send()
        .await;

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
