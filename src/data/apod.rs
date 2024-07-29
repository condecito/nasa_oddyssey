use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Lens, Debug,Deserialize,Serialize)]
pub struct ApodView {
    #[serde(default)]
    pub copyright: String,
    pub date: String,
    pub explanation: String,
    pub media_type: String,
    pub title: String,
    #[serde(default)]
    pub hdurl: String,
    pub url: String,
}