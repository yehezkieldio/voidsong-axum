use axum::routing::get;
use axum::Router;

use crate::{
    handlers::{random_humor_handler, random_media_handler, random_trivia_handler},
    utils::state::AppState,
};

pub fn routes() -> Router {
    let state: AppState = AppState::new();

    let trivia = Router::new()
        .route("/cat", get(random_trivia_handler::cat_fact))
        .with_state(state.clone());

    let humor = Router::new()
        .route("/chucknorris", get(random_humor_handler::chuck_norris))
        .route("/dadjoke", get(random_humor_handler::dad_joke))
        .with_state(state.clone());

    let media = Router::new()
        .route("/cat", get(random_media_handler::cat))
        .route("/dog", get(random_media_handler::dog))
        .route("/fox", get(random_media_handler::fox))
        .route("/bunny", get(random_media_handler::bunny))
        .route("/duck", get(random_media_handler::duck))
        .with_state(state.clone());

    let router = Router::new()
        .nest("/trivia", trivia)
        .nest("/media", media)
        .nest("/humor", humor);

    router
}
