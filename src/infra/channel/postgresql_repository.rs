use std::convert::TryFrom;
use std::sync::Arc;

use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use tokio_postgres::{Client, Row};

use crate::domain::channel::model::{Channel, DraftChannel};
use crate::domain::channel::repository::ChannelRepository;
use crate::domain::url::Url;

#[derive(Clone)]
pub struct PostgreSQLChannelRepository {
    client: Arc<Client>,
}

impl TryFrom<&Row> for Channel {
    type Error = Error;

    fn try_from(value: &Row) -> Result<Self> {
        Ok(Channel {
            id: value.try_get("id")?,
            name: value.try_get("name")?,
        })
    }
}

impl PostgreSQLChannelRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgreSQLChannelRepository { client }
    }
}

#[async_trait]
impl ChannelRepository for PostgreSQLChannelRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Channel>> {
        let result = self
            .client
            .query_one(r#"SELECT * FROM channels WHERE id=$1;"#, &[&id])
            .await?;

        match result.is_empty() {
            true => Ok(None),
            false => Ok(Some(Channel::try_from(&result)?)),
        }
    }
    async fn create(&self, channel: DraftChannel) -> Result<()> {
        let result = self
            .client
            .execute(
                r#"INSERT INTO channels(id, name) VALUES ($1, $2);"#,
                &[&channel.id, &channel.name],
            )
            .await?;
        match result {
            0 => Err(anyhow!("Failed Insert row.data: {:?}", channel)),
            _ => Ok(()),
        }
    }

    async fn bulk_register(&self, channels: Vec<DraftChannel>) -> Result<()> {
        for c in channels {
            self.client.execute(r#"INSERT INTO channels(id, name) VALUES ($1, $2);"#, &[&c.id, &c.name]).await;
        }
        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(not(feature = "integration_test"), cfg(ignore))]
mod integration_test {
    use super::*;
    use dotenv::dotenv;
    use std::{collections::HashMap, env::vars};
    use tokio::spawn;
    use tokio_postgres::{connect, Client, NoTls};

    async fn teardown(client: Arc<Client>) {
        client
            .execute(r#"DELETE FROM "channels";"#, &[])
            .await
            .expect("Failed clean up channels table");
    }

    #[tokio::test]
    async fn create_add_row_and_find_by_id() {
        dotenv().ok();
        let envs: HashMap<_, _> = vars().collect();
        let db_config = envs
            .get("TESTING_DATABASE_URL")
            .expect("TESTING_DATABASE_URL must be set");

        let (client, pg_connection) = connect(db_config, NoTls).await.unwrap();
        let a_client = Arc::new(client);

        spawn(async move { pg_connection.await });
        let repository = PostgreSQLChannelRepository::new(a_client.clone());
        let draft_channel = DraftChannel {
            id: "foo".to_string(),
            name: "bar".to_string(),
        };
        repository
            .create(draft_channel)
            .await
            .expect("Failed create draft channel");
        let channel = repository
            .find_by_id("foo")
            .await
            .expect("foo is not found in channels");
        assert!(channel.is_some());
        teardown(a_client).await;
    }

    #[tokio::test]
    async fn bulk_register() {
        dotenv().ok();
        let envs: HashMap<_, _> = vars().collect();
        let db_config = envs
            .get("TESTING_DATABASE_URL")
            .expect("TESTING_DATABASE_URL must be set");

        let (client, pg_connection) = connect(db_config, NoTls).await.unwrap();
        let a_client = Arc::new(client);

        spawn(async move { pg_connection.await });
        let repository = PostgreSQLChannelRepository::new(a_client.clone());
        let draft_channel = DraftChannel {
            id: "foo".to_string(),
            name: "bar".to_string(),
        };
        repository
            .bulk_register(vec![draft_channel, DraftChannel {  id: "poe".to_string(), name: "poepoe".to_string() }])
            .await
            .expect("Failed create draft channel");
        let channel = repository
            .find_by_id("foo")
            .await
            .expect("foo is not found in channels");
        assert!(channel.is_some());
        let channel1 = repository
            .find_by_id("poe")
            .await
            .expect("poe is not found in channels");
        assert!(channel.is_some());
        teardown(a_client).await;
    }
}
