use axum::{routing::get, Router};

use crate::handlers::random_handler;

pub fn animal() -> Router {
    let router = Router::new()
        .route("/cat", get(random_handler::get_random_cat))
        .route("/dog", get(random_handler::get_random_dog));

    router
}
