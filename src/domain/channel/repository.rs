use crate::domain::channel::model::{Channel, DraftChannel};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelRepository
where
    Self: Sized + Clone + Send + Sync,
{
    async fn find_by_id(&self, id: &str) -> Result<Option<Channel>>;
    async fn search_by_name(&self, name: &str) -> Result<Vec<Channel>>;
    async fn create(&self, channel: DraftChannel) -> Result<()>;
}
