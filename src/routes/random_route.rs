use axum::{routing::get, Router};

use crate::handlers::random_handler;

pub fn routes() -> Router {
    let animals = Router::new()
        .route("/cat", get(random_handler::get_random_cat))
        .route("/dog", get(random_handler::get_random_dog));

    let router = Router::new().nest("/animal", animals);

    router
}
