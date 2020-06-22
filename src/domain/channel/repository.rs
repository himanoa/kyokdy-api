use crate::domain::channel::model::{Channel, DraftChannel};
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait ChannelRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Channel>>;
    async fn search_by_name(&self, name: &str) -> Result<Vec<Channel>>;
    async fn create(&self, channel: DraftChannel) -> Result<()>;
}
