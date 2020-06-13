use chrono::Utc;

pub struct Unixtime(pub i64);

impl Unixtime {
    pub fn now() -> Self{
        Unixtime(Utc::now().timestamp())
    }
}
