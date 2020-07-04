use super::model::Song;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SongRepository {
    async fn search(
        &self,
        title: Option<&str>,
        channel_name: Option<&str>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Song>>;
}
