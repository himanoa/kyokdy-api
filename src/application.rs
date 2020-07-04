use crate::infra::channel::postgresql_repository::PostgreSQLChannelRepository;
use crate::infra::song::postgresql_repository::PostgreSQLSongRepository;
use crate::infra::video::postgresql_repository::PostgreSQLVideoRepository;
use crate::{domain, IApplication};
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Clone)]
pub struct Application {
    client: Arc<Client>,
}

impl IApplication for Application {
    fn channel_repository(
        &self,
    ) -> Arc<dyn domain::channel::repository::ChannelRepository + Send + Sync> {
        Arc::new(PostgreSQLChannelRepository::new(self.client.clone()))
    }
    fn video_repository(
        &self,
    ) -> Arc<dyn domain::video::repository::VideoRepository + Send + Sync> {
        Arc::new(PostgreSQLVideoRepository::new(self.client.clone()))
    }

    fn song_repository(&self) -> Arc<dyn domain::song::repository::SongRepository + Send + Sync> {
        Arc::new(PostgreSQLSongRepository::new(self.client.clone()))
    }

    fn video_service(&self) -> Arc<domain::video::service::VideoService> {
        Arc::new(domain::video::service::VideoService::new(
            self.video_repository(),
        ))
    }
}

impl Application {
    pub fn new(client: Client) -> Self {
        Application {
            client: Arc::new(client),
        }
    }
}
