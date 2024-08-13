mod handlers;
mod routes;
mod utils;

use std::env;

use axum::{extract, ServiceExt};
use dotenvy::dotenv;
use tokio::net::TcpListener;

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: &str = "8080";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (host, port) = env();

    let routes = routes::root::routes();
    let listener: TcpListener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    println!("Server is running on {}:{}", host, port);
    axum::serve(
        listener,
        ServiceExt::<extract::Request>::into_make_service(routes),
    )
    .await
    .unwrap();
}

fn env() -> (String, String) {
    dotenv().expect("Failed to load .env file");

    let host: String = env::var("SERVER_HOST").unwrap_or_else(|_| SERVER_HOST.to_string());
    let port: String = env::var("SERVER_PORT").unwrap_or_else(|_| SERVER_PORT.to_string());

    (host, port)
}
