use axum::routing::get;
use axum::Router;

use crate::{
    handlers::{random_fact_handler, random_image_handler},
    utils::state::AppState,
};

pub fn routes() -> Router {
    let state: AppState = AppState::new();

    let animals: Router = Router::new()
        .route("/cat", get(random_image_handler::cat))
        .route("/dog", get(random_image_handler::dog))
        .route("/fox", get(random_image_handler::fox))
        .route("/bunny", get(random_image_handler::bunny))
        .route("/duck", get(random_image_handler::duck))
        .with_state(state.clone());

    let facts: Router = Router::new()
        .route("/cat", get(random_fact_handler::cat_fact))
        .route("/chucknorris", get(random_fact_handler::chuck_norris))
        .with_state(state.clone());

    let router: Router = Router::new().nest("/animal", animals).nest("/fact", facts);

    router
}
