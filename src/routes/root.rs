use axum::{routing::IntoMakeService, Router};

use super::random_route;

pub fn routes() -> IntoMakeService<Router> {
    let app_router = Router::new().nest("/api", random_route::routes());

    app_router.into_make_service()
}
