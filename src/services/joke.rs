use reqwest::{Client, Error};
use serde::Deserialize;

const JOKE_API: &str = "https://v2.jokeapi.dev/joke/Any?format=json&type=single&lang=en&amount=1";

#[derive(Debug, Deserialize)]
pub struct JokeApi {
    pub id: usize,
    pub category: String,
    pub joke: String,
}

pub async fn joke_from_joke_api() -> Result<JokeApi, Error> {
    let resp = Client::new()
        .get(JOKE_API)
        .send()
        .await?
        .json::<JokeApi>()
        .await?;

    Ok(resp)
}
