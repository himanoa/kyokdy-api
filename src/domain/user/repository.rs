use crate::domain::user::model::User;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository
where
    Self: Sized + Clone + Send + Sync,
{
    async fn create(&self, user: User) -> Result<()>;
    async fn find_by_uuid(&self, uuid: &str) -> Result<Option<User>>;
}
