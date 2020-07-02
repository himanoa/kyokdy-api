use std::convert::TryFrom;
use std::sync::Arc;

use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use tokio_postgres::{Client, Row};

use crate::domain::video::model::{Video, VideoId, DraftVideo};
use crate::domain::video::repository::VideoRepository;
use crate::domain::url::Url;


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


#[async_trait]
impl VideoRepository for PostgreSQLVideoRepository {
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Video>>{
      let result = self.client.query(r#"SELECT * FROM videos LIMIT $1 OFFSET $2;"#, &[&limit, &offset]).await?;
      Ok(result
          .iter()
          .filter_map(|row| Video::try_from(row).ok())
          .collect::<Vec<Video>>())

    }
    async fn listByChannel(&self, channel_id: i64, limit: i64, offset: i64) -> Result<Vec<Video>>{
      let result = self.client.query(r#"SELECT * FROM videos WHERE `channel_id` = $1 LIMIT $2 OFFSET $3;"#, &[&channel_id, &limit, &offset]).await?;
      Ok(result
          .iter()
          .filter_map(|row| Video::try_from(row).ok())
          .collect::<Vec<Video>>())

    }

    async fn create(&self, video: DraftVideo) -> Result<()> {
        unimplemented!()
        // self.client.execute(r#"INSERT INTO videos(title, thumbnail_url, url)"#, [])
    }
}

#[cfg(test)]
#[cfg_attr(not(feature = "integration_test"), cfg(ignore))]
mod integration_test {}