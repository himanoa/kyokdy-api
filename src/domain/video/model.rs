use crate::domain::url::Url;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Video { 
    pub id: String,
    pub title: String,
    pub thumbnail_url: Url,
    pub url: Url,
}

impl Video {
    pub fn new(id: String, title: String, thumbnail_url: Url, url: Url) -> Self {
        Video { id, title, thumbnail_url, url}
    }
}
