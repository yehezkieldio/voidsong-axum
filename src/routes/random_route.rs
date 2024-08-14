use axum::routing::get;
use axum::Router;

use crate::handlers::random_handler;
use crate::utils::state::AppState;

pub fn routes() -> Router {
    let state: AppState = AppState::new();

    let animals: Router = Router::new()
        .route("/cat", get(random_handler::get_random_cat))
        .route("/dog", get(random_handler::get_random_dog))
        .with_state(state);

    let router: Router = Router::new().nest("/animal", animals);

    router
}
