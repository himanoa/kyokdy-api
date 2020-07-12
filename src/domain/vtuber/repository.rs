use crate::domain::channel::model::DraftChannel;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait VTuberRepository {
    async fn list(&self) -> Result<Vec<DraftChannel>>;
}
