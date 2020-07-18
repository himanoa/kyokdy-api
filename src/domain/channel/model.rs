use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct DraftChannel {
    pub id: String,
    pub name: String,
}
