use crate::domain::song::model::Song;
use crate::domain::song::repository::SongRepository;
use crate::domain::video::model::VideoId;
use crate::infra::postgresql_helper::escape_like_query;
use anyhow::{Error, Result};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio_postgres::{Client, Row};

impl TryFrom<&Row> for Song {
    type Error = Error;
    fn try_from(value: &Row) -> Result<Self, anyhow::Error> {
        Ok(Song {
            id: value.try_get("id")?,
            video_id: VideoId(value.try_get("video_id")?),
            title: value.try_get("title")?,
            start_timestamp: value.try_get("start_timestamp")?,
            end_timestamp: value.try_get("end_timestamp")?,
        })
    }
}

fn generate_like_query(s: String) -> String {
    format!("%{}%", escape_like_query(s))
}

#[derive(Clone)]
pub struct PostgreSQLSongRepository {
    client: Arc<Client>,
}

impl PostgreSQLSongRepository {
    pub fn new(client: Arc<Client>) -> Self {
        PostgreSQLSongRepository { client }
    }
}

#[async_trait]
impl SongRepository for PostgreSQLSongRepository {
    async fn search(
        &self,
        title: Option<String>,
        channel_name: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Song>> {
        let (title_query, channel_name_query) = (
            title.map(generate_like_query),
            channel_name.map(generate_like_query),
        );
        let result = match (title_query, channel_name_query) {
            (None, None) => {
                self.client
                    .query(
                        r#"SELECT * FROM songs LIMIT $1 OFFSET $2;"#,
                        &[&limit, &offset],
                    ).await?
            }
            (Some(tq), None) => {
                self.client
                    .query(
                        r#"SELECT * FROM songs WHERE title LIKE $1 LIMIT $2 OFFSET $3;"#,
                        &[&tq, &limit, &offset]
                    ).await?
            }
            (None, Some(cnq)) => {
                self.client
                    .query(
                        r#"SELECT * FROM songs INNER JOIN videos ON songs.video_id = videos.id INNER JOIN channels ON videos.channel_id = channels.id WHERE channels.name LIKE $1 LIMIT $2 OFFSET $3;"#,
                        &[&cnq, &limit, &offset]
                    ).await?
            }
            (Some(tq), Some(cnq)) => {
                self.client
                    .query(
                        r#"SELECT * FROM songs INNER JOIN videos ON songs.video_id = videos.id INNER JOIN channels ON videos.channel_id = channels.id WHERE channels.name LIKE $1 AND songs.title LIKE $2 LIMIT $3 OFFSET $4;"#,
                        &[&cnq, &tq, &limit, &offset]
                    ).await?
            }
        };
        Ok(result
            .iter()
            .flat_map(Song::try_from)
            .collect::<Vec<Song>>())
    }
}
