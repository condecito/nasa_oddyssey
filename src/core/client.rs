use reqwest::{Error, StatusCode};
use app_properties::AppProperties;
use serde_json::json;
use crate::data::apod::ApodView;
use crate::data::launcher::LauncherView;

pub async fn apod() -> Result<ApodView, Error> {
    let properties: AppProperties = AppProperties::new();
    let mut apod_url = properties.get("apod").to_owned();
    let api_key = properties.get("key");
    apod_url.push_str("?api_key=");
    apod_url.push_str(&api_key);
    println!("URl={:?}", apod_url);
    let response = reqwest::get(apod_url).await?;
    let text = response.text().await.unwrap();
    let v:ApodView = serde_json::from_str(&text).unwrap();
    return Result::Ok(v);
}

