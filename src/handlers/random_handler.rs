use axum::response::IntoResponse;
use reqwest::{header, Client, Error, Response as ReqwestResponse, StatusCode};

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
