use crate::domain::video::model::VideoId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Song {
    pub id: i64,
    pub video_id: VideoId,
    pub title: String,
    pub start_timestamp: i32,
    pub end_timestamp: i32,
}

impl Song {
    pub fn new(
        id: i64,
        video_id: VideoId,
        title: String,
        start_timestamp: i32,
        end_timestamp: i32,
    ) -> Self {
        Song {
            id,
            video_id,
            title,
            start_timestamp,
            end_timestamp,
        }
    }
}
