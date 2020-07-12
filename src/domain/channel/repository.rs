use crate::domain::channel::model::{Channel, DraftChannel};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Channel>>;
    async fn create(&self, channel: DraftChannel) -> Result<()>;
    async fn bulk_register(&self, channels: Vec<DraftChannel>) -> Result<()>;
}

