use crate::domain::vtuber::repository::VTuberRepository;
use anyhow::Result;
use log::info;
use std::sync::Arc;

pub struct BulkRegisterService {
    vtuber_repository: Arc<dyn VTuberRepository>,
}

impl BulkRegisterService {
    pub fn new(repository: Arc<dyn VTuberRepository>) -> Self {
        BulkRegisterService {
            vtuber_repository: repository,
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
        for dc in draft_channels {
            info!("{:?}", dc)
        }
        Ok(())
    }
}
