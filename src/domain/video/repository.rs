use super::model::{DraftVideo, Video};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait VideoRepository {
    async fn list_by_channel(
        &self,
        channel_id: String,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Video>>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Video>>;
    async fn create(&self, video: DraftVideo) -> Result<()>;
}
