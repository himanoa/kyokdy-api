use crate::domain::user::model::User;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<()>;
    async fn find_by_uuid(&self, uuid: &str) -> Result<Option<User>>;
}
