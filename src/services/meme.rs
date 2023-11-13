use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MemeApi {
    pub title: String,
    pub url: String,
    pub preview: Vec<String>,
}

pub async fn meme_from_api() -> Result<MemeApi, Error> {
    let url = "https://meme-api.com/gimme";

    let resp = Client::new()
        .get(url)
        .send()
        .await?
        .json::<MemeApi>()
        .await?;

    Ok(resp)
}

pub async fn get_meme() -> Result<String, Error> {
    let mut res = meme_from_api().await?;

    let image_url = match res.preview.pop() {
        Some(url) => url,
        None => res.url,
    };

    Ok(image_url)
}
