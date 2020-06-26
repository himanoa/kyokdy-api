use crate::infra::channel::postgresql_repository::PostgreSQLChannelRepository;
use crate::{infra, IApplication, domain};
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Clone)]
pub struct Application {
    client: Arc<Client>,
}

impl IApplication for Application {
    fn channel_repository(&self) -> Arc<dyn domain::channel::repository::ChannelRepository + Send + Sync> {
        Arc::new(PostgreSQLChannelRepository::new(self.client.clone()))
    }
}

impl Application {
    pub fn new(client: Client) -> Self {
        Application {
            client: Arc::new(client),
        }
    }
}
