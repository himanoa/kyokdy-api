use crate::domain::url::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub icon_url: Url,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DraftChannel {
    pub id: String,
    pub name: String,
    pub icon_url: Url,
}
