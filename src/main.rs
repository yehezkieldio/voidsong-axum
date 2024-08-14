use axum::{extract, ServiceExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let (host, port) = voidsong::env::load();

    let routes = voidsong::routes::root::routes();
    let listener: TcpListener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    tracing::info!("Server running on http://{}:{}", host, port);
    axum::serve(
        listener,
        ServiceExt::<extract::Request>::into_make_service(routes),
    )
    .await
    .unwrap();
}
