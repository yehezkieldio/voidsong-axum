use axum::{routing::get, Router};

use crate::{handlers::random_handler, utils::state::AppState};

pub fn routes() -> Router {
    let state = AppState::new();

    let animals = Router::new()
        .route("/cat", get(random_handler::get_random_cat))
        .route("/dog", get(random_handler::get_random_dog))
        .with_state(state);

    let router = Router::new().nest("/animal", animals);

    router
}
