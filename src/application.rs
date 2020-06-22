use std::sync::Arc;
use tokio_postgres::Client;
use crate::{IApplication, infra };
use crate::infra::channel::postgresql_repository::PostgreSQLChannelRepository;

#[derive(Clone)]
pub struct Application {
    client: Arc<Client>
}

impl IApplication for Application {
    type ChannelRepository = infra::channel::postgresql_repository::PostgreSQLChannelRepository;
    fn channel_repository(&self) -> Self::ChannelRepository  {
        PostgreSQLChannelRepository::new(self.client.clone())
    }
}

impl Application {
    pub fn new(client: Client) -> Self {
        Application {
            client: Arc::new(client)
        }
    }
}
