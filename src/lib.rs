pub mod application;
pub mod domain;
pub mod exception;
pub mod handlers;
pub mod infra;
pub mod initializer;
pub mod routes;

use std::sync::Arc;

pub struct Config<'a> {
    pub db_url: &'a str,
}

pub trait IApplication {
    fn channel_repository(
        &self,
    ) -> Arc<dyn domain::channel::repository::ChannelRepository + Send + Sync>;

    fn video_repository(&self)
        -> Arc<dyn domain::video::repository::VideoRepository + Send + Sync>;

    fn song_repository(&self) -> Arc<dyn domain::song::repository::SongRepository + Send + Sync>;
    fn video_service(&self) -> Arc<domain::video::service::VideoService>;
}
