use axum::Router;
use tower_http::{
    normalize_path::{NormalizePath, NormalizePathLayer},
    trace::{self, TraceLayer},
};
use tower_layer::Layer;
use tracing::Level;

use super::random_route;

pub fn routes() -> NormalizePath<Router> {
    let app_router = Router::new().nest("/random", random_route::routes()).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    NormalizePathLayer::trim_trailing_slash().layer(app_router)
}
