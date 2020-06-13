use crate::domain::url::Url;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Channel { 
    pub id: String,
    pub name: String,
    pub icon_url: Url,
}

impl Channel {
    pub fn new(id: String, name: String, icon_url: Url) -> Self {
        Channel {
            id, name, icon_url
        }
    }
}
