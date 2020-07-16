use crate::domain::video::model::VideoId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Song {
    pub id: i64,
    pub video_id: VideoId,
    pub title: String,
    pub start_timestamp: i32,
    pub end_timestamp: Option<i32>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct DraftSong {
    pub video_id: VideoId,
    pub title: String,
    pub start_timestamp: i32,
    pub end_timestamp: Option<i32>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct SearchSongParams {
    pub title: Option<String>,
    pub channel_name: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

impl Song {
    pub fn new(
        id: i64,
        video_id: VideoId,
        title: String,
        start_timestamp: i32,
        end_timestamp: Option<i32>,
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
