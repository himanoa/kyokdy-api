use crate::domain::url::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct VideoId(pub String);

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Video {
    pub id: VideoId,
    pub title: String,
    pub thumbnail_url: Url,
    pub url: Url,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct DraftVideo {
    pub channel_id: i64,
    pub title: String,
    pub thumbnail_url: Url,
    pub url: Url,
}

impl Video {
    pub fn new(id: String, title: String, thumbnail_url: Url, url: Url) -> Self {
        Video {
            id: VideoId(id),
            title,
            thumbnail_url,
            url,
        }
    }
}
