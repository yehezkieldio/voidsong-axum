use std::env;

use dotenvy::dotenv;

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: &str = "8080";

pub fn load() -> (String, String) {
    // load .env file if it exists if not use default values
    if dotenv().is_err() {
        return (SERVER_HOST.to_string(), SERVER_PORT.to_string());
    }

    let host: String = env::var("SERVER_HOST").unwrap_or_else(|_| SERVER_HOST.to_string());
    let port: String = env::var("SERVER_PORT").unwrap_or_else(|_| SERVER_PORT.to_string());

    (host, port)
}
