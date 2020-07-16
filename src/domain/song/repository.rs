use super::model::{Song, DraftSong};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SongRepository {
    async fn search(
        &self,
        title: Option<String>,
        channel_name: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Song>>;
    async fn create(&self, draft_song: DraftSong) -> Result<()>;
}
