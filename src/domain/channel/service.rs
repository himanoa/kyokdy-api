use super::model::DraftChannel;
use super::repository::ChannelRepository;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ChannelService {
    channel_repository: Arc<dyn ChannelRepository + Send + Sync>,
}

impl ChannelService {
    pub async fn append_channel(&self, draft_channel: DraftChannel) -> Result<()> {
        self.channel_repository.create(draft_channel).await
    }
}
