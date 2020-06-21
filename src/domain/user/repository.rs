use crate::domain::user::model::User;
use crate::exception::DataBaseError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<(), DataBaseError>;
    async fn find_by_uuid(&self, uuid: &str) -> Result<Option<User>, DataBaseError>;
}
