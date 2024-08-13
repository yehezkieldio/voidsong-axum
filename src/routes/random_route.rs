use axum::{routing::get, Router};

use crate::handlers::random_handler;

pub fn routes() -> Router {
    let router = Router::new()
        .route("/cats", get(random_handler::get_random_cat))
        .route("/dogs", get(random_handler::get_random_dog));

    router
}
