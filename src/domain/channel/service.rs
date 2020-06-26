use super::repository::ChannelRepository;
use super::model::DraftChannel;
use std::sync::Arc;
use async_trait::async_trait;
use anyhow::Result;

pub struct ChannelService
{
    channel_repository: Arc<dyn ChannelRepository + Send + Sync>,
}


impl ChannelService  {
    pub async fn append_channel(&self, draft_channel : DraftChannel) -> Result<()> {
        self.
            channel_repository
            .clone()
            .create(draft_channel).await
    }
}
