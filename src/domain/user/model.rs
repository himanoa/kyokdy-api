use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct User {
    pub id: i64,
    pub uid: String,
}

