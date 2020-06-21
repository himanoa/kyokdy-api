use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Unixtime(pub i64);

impl Unixtime {
    pub fn now() -> Self {
        Unixtime(Utc::now().timestamp())
    }
}
