use reqwest::Client;
use tokio::sync::MutexGuard;

use super::{response::VoidsongImage, state::user_agent};

pub async fn preflight_check<'a>(
    client: &MutexGuard<'a, Client>,
    urls: Vec<&'a str>,
) -> (bool, Option<&'a str>) {
    for url in urls {
        let response = client.head(url).headers(user_agent()).send().await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    return (true, Some(url));
                }
            }
            Err(_) => continue,
        }
    }
    (false, None)
}

pub async fn fetch_image<'a>(client: MutexGuard<'a, Client>, url: &str) -> Option<VoidsongImage> {
    let response = client.get(url).headers(user_agent()).send().await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let content_type = response
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or("image/jpeg")
                    .to_string();

                let bytes = response.bytes().await;

                match bytes {
                    Ok(bytes) => Some(VoidsongImage {
                        content_type,
                        bytes: bytes.to_vec(),
                    }),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
