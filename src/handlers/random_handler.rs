use axum::http::Response;

pub fn get_random_cat() -> Response<String> {
    let response_builder = Response::builder();

    response_builder
        .status(200)
        .body("Random cat".into())
        .unwrap()
}
