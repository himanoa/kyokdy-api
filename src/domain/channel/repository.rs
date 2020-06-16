use async_trait::async_trait;
use crate::domain::channel::model::Channel;
use crate::exception::DataBaseError;


#[async_trait]
pub trait ChannelRepository {
    async fn find_by_id(id: &str) -> Result<Channel, DataBaseError>;
    async fn search_by_title(title: &str) -> Result<Channel, DataBaseError>;
    async fn create(channel: Channel) -> Result<(), DataBaseError>;
}
