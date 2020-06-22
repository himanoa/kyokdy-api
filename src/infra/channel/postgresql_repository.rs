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
        let icon_url: String = value.try_get("icon_url")?;

        Ok(Channel {
            id: value.try_get("id")?,
            channel_id: value.try_get("channel_id")?,
            name: value.try_get("name")?,
            icon_url: Url::try_from(icon_url)?,
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
            .query_one(r#"SELECT * FROM channels WHERE channel_id=$1;"#, &[&id])
            .await?;

        match result.is_empty() {
            true => Ok(None),
            false => Channel::try_from(&result).map_or(Ok(None), |channel| Ok(Some(channel))),
        }
    }

    async fn search_by_name(&self, title: &str) -> Result<Vec<Channel>> {
        let rows = self
            .client
            .query(
                r#"SELECT * FROM channels WHERE name LIKE '%$1%';"#,
                &[&title],
            )
            .await?;
        rows.iter().try_fold(vec![], |mut channels, row| {
            if let Ok(channel) = Channel::try_from(row) {
                channels.push(channel);
            }
            Ok(channels)
        })
    }

    async fn create(&self, channel: DraftChannel) -> Result<()> {
        let result = self
            .client
            .execute(
                r#"INSERT INTO channels(channel_id, name, icon_url) VALUES ($1, $2, $3);"#,
                &[&channel.channel_id, &channel.name, &channel.icon_url.0],
            )
            .await?;
        match result {
            0 => Err(anyhow!("Failed Insert row.data: {:?}", channel)),
            _ => Ok(()),
        }
    }
}
