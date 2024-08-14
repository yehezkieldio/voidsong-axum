use std::env;

use dotenvy::dotenv;

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: &str = "8080";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn load() -> (String, String) {
    let _ = dotenv();

    let host: String = env::var("SERVER_HOST").unwrap_or_else(|_| SERVER_HOST.to_string());
    let port: String = env::var("SERVER_PORT").unwrap_or_else(|_| SERVER_PORT.to_string());

    (host, port)
}
