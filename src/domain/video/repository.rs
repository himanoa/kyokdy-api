use super::model::{DraftVideo, Video};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait VideoRepository {
    async fn list_by_channel(&self, channel_id: i64, limit: i64, offset: i64) -> Result<Vec<Video>>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Video>>;
    async fn create(&self, video: DraftVideo) -> Result<()>;
}
