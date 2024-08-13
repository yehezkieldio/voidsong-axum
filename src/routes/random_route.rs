use axum::Router;

use crate::handlers::random_handler;

pub fn routes() -> Router {
    let router = Router::new().route(
        "/cats",
        axum::routing::get(random_handler::get_random_cat()),
    );

    return router;
}
