use crate::domain::channel::repository::ChannelRepository;
use crate::domain::vtuber::repository::VTuberRepository;
use anyhow::Result;
use log::info;
use std::sync::Arc;

pub struct BulkRegisterService {
    vtuber_repository: Arc<dyn VTuberRepository>,
    channel_repository: Arc<dyn ChannelRepository>,
}

impl BulkRegisterService {
    pub fn new(
        vtuber_repository: Arc<dyn VTuberRepository>,
        channel_repository: Arc<dyn ChannelRepository>,
    ) -> Self {
        BulkRegisterService {
            vtuber_repository: vtuber_repository,
            channel_repository: channel_repository,
        }
    }

    pub async fn dry_run(&self) -> Result<()> {
        let draft_channels = self.vtuber_repository.list().await?;
        for dc in draft_channels {
            info!("{:?}", dc)
        }
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let draft_channels = self.vtuber_repository.list().await?;
        self.channel_repository
            .bulk_register(draft_channels)
            .await?;
        Ok(())
    }
}
