use std::convert::TryFrom;
use std::sync::Arc;

use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use tokio_postgres::{Client, Row};

use crate::domain::url::Url;
use crate::domain::video::model::{DraftVideo, Video, VideoId};
use crate::domain::video::repository::VideoRepository;

impl TryFrom<&Row> for Video {
    type Error = Error;

    fn try_from(value: &Row) -> Result<Self> {
        let thumbnail_url: String = value.try_get("thumbnail_url")?;
        let url: String = value.try_get("url")?;
        let id: VideoId = VideoId(value.try_get("video_id")?);

        Ok(Video {
            id,
            title: value.try_get("title")?,
            thumbnail_url: Url::try_from(thumbnail_url)?,
            url: Url::try_from(url)?,
        })
    }
}

#[derive(Clone)]
pub struct PostgreSQLVideoRepository {
    client: Arc<Client>,
}

impl PostgreSQLVideoRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgreSQLVideoRepository { client }
    }
}

#[async_trait]
impl VideoRepository for PostgreSQLVideoRepository {
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Video>> {
        let result = self
            .client
            .query(
                r#"SELECT * FROM videos LIMIT $1 OFFSET $2;"#,
                &[&limit, &offset],
            )
            .await?;
        Ok(result
            .iter()
            .flat_map(|row| Video::try_from(row))
            .collect::<Vec<Video>>())
    }
    async fn list_by_channel(
        &self,
        channel_id: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Video>> {
        let result = self
            .client
            .query(
                r#"SELECT * FROM videos WHERE channel_id = $1 LIMIT $2 OFFSET $3;"#,
                &[&channel_id, &limit, &offset],
            )
            .await?;
        Ok(result
            .iter()
            .flat_map(|row| Video::try_from(row))
            .collect::<Vec<Video>>())
    }

    async fn bulk_create(&self, videos: Vec<DraftVideo>) -> Result<()> {
        for video in videos {
            self.client.execute(r#"INSERT INTO videos(id, channel_id, title, thumbnail_url, url) VALUES ($1, $2, $3, $4, $5);"#, &[&video.id.0, &video.channel_id, &video.title, &video.thumbnail_url.0, &video.url.0]).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(not(feature = "integration_test"), cfg(ignore))]
mod integration_test {
    use super::*;
    use dotenv::dotenv;
    use std::sync::Arc;
    use std::{collections::HashMap, env::vars};
    use crate::infra::channel::postgresql_repository::PostgreSQLChannelRepository;
    use crate::domain::video::model::VideoId;
    use crate::domain::channel::model::DraftChannel;
    use crate::domain::channel::repository::ChannelRepository;
    use crate::domain::url::Url;
    use tokio::spawn;
    use tokio_postgres::{connect, Client, NoTls};

    async fn teardown(client: Arc<Client>) {
        client
            .execute(r#"DELETE FROM "videos";"#, &[])
            .await
            .expect("Failed clean up channels table");
        client
            .execute(r#"DELETE FROM "channels";"#, &[])
            .await
            .expect("Failed clean up channels table");
    }

    #[tokio::test]
    async fn test_bulk_register() {
        dotenv().ok();
        let envs: HashMap<_, _> = vars().collect();
        let db_config = envs
            .get("TESTING_DATABASE_URL")
            .expect("TESTING_DATABASE_URL must be set");

        let (client, pg_connection) = connect(db_config, NoTls).await.unwrap();
        let a_client = Arc::new(client);

        spawn(async move { pg_connection.await });

        let channel_repository = PostgreSQLChannelRepository::new(a_client.clone());
        let video_repository = PostgreSQLVideoRepository::new(a_client.clone());

        let draft_channel = DraftChannel {
            id: "foo".to_string(),
            name: "bar".to_string(),
        };

        channel_repository.bulk_register(vec![draft_channel.clone()]).await;

        let videos = vec![DraftVideo {
            id: VideoId("foo".to_string()),
            channel_id: draft_channel.clone().id,
            title: "foo".to_string(),
            url: Url::try_from("https://example.com".to_string()).unwrap(),
            thumbnail_url: Url::try_from("https://example.com".to_string()).unwrap()
        }, DraftVideo {
            id: VideoId("fooa".to_string()),
            channel_id: draft_channel.clone().id,
            title: "foo".to_string(),
            url: Url::try_from("https://example.com".to_string()).unwrap(),
            thumbnail_url: Url::try_from("https://example.com".to_string()).unwrap()
        }];

        video_repository.bulk_create(videos.clone()).await;

        let actual = video_repository.list_by_channel(draft_channel.clone().id, 100, 0).await.unwrap();

        for (i, a) in actual.iter().enumerate() {
            assert_eq!(a.id, videos.get(i).unwrap().id);
        }

        teardown(a_client);
    }
}
