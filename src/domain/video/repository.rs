use super::model::{DraftVideo, Video};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait VideoRepository {
    async fn list_by_channel(
        &self,
        channel_id: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Video>>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Video>>;
    async fn bulk_create(&self, videos: Vec<DraftVideo>) -> Result<()>;
}
