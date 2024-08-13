use axum::response::IntoResponse;
use reqwest::{header, Client, StatusCode};

pub async fn get_random_cat() -> Result<impl IntoResponse, impl IntoResponse> {
    let client = Client::new();
    let url = "https://cataas.com/cat";

    let res = client.get(url).send().await.unwrap();

    match res.status() {
        StatusCode::OK => Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/jpeg")],
            res.bytes().await.unwrap(),
        )),
        _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch random cat image",
        )
            .into_response()),
    }
}
