use crate::domain::channel::model::Channel;
use crate::exception::DataBaseError;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Channel>, DataBaseError>;
    async fn search_by_title(&self, title: &str) -> Result<Vec<Channel>, DataBaseError>;
    async fn create(&self, channel: Channel) -> Result<(), DataBaseError>;
}
