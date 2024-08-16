use axum::extract::State;
use reqwest::{Client, Response};
use serde::Deserialize;

use crate::utils::{
    response::{VoidsongError, VoidsongFact},
    state::{user_agent, AppState},
    url::preflight_check,
};

#[derive(Deserialize)]
struct CatFactNinja {
    fact: String,
}

pub async fn get_random_cat_fact(
    State(state): State<AppState>,
) -> Result<VoidsongFact, VoidsongError> {
    let urls: Vec<&str> = vec!["https://catfact.ninja/fact"];
    let client: Client = state.client;

    // Check if the APIs are available
    let (success, url) = preflight_check(&client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match client.get(url.unwrap()).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchFact),
    };

    let fact: String = match get_url.json::<CatFactNinja>().await {
        Ok(response) => response.fact,
        Err(_) => return Err(VoidsongError::FailedToFetchFact),
    };

    Ok(VoidsongFact { fact })
}

/* -------------------------------------------------------------------------- */

#[derive(Deserialize)]
struct ChuckNorrisFact {
    value: String,
}

pub async fn get_random_chucknorris_fact(
    State(state): State<AppState>,
) -> Result<VoidsongFact, VoidsongError> {
    let urls: Vec<&str> = vec!["https://api.chucknorris.io/jokes/random"];
    let client: Client = state.client;

    // Check if the APIs are available
    let (success, url) = preflight_check(&client, urls).await;
    if !success {
        return Err(VoidsongError::ServiceUnavailable);
    }

    // Get the image URL
    let get_url: Response = match client.get(url.unwrap()).headers(user_agent()).send().await {
        Ok(response) => response,
        Err(_) => return Err(VoidsongError::FailedToFetchFact),
    };

    let fact: String = match get_url.json::<ChuckNorrisFact>().await {
        Ok(response) => response.value,
        Err(_) => return Err(VoidsongError::FailedToFetchFact),
    };

    Ok(VoidsongFact { fact })
}
