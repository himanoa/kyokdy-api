use super::model::Video;
use super::repository::VideoRepository;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct VideoService {
    video_repository: Arc<dyn VideoRepository + Send + Sync>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListVideoParameter {
    offset: u32,
    limit: u32,
    channel_id: Option<String>,
}

impl VideoService {
    pub fn new(repository: Arc<dyn VideoRepository + Send + Sync>) -> Self {
        VideoService {
            video_repository: repository,
        }
    }
    pub async fn list(&self, params: ListVideoParameter) -> Result<Vec<Video>> {
        if let Some(channel_id) = params.channel_id {
            self.video_repository
                .list_by_channel(channel_id, params.limit, params.offset)
                .await
        } else {
            self.video_repository
                .list(params.limit, params.offset)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::url::Url;
    use crate::domain::video::model::DraftVideo;
    use crate::domain::video::repository::VideoRepository;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use std::convert::TryFrom;

    #[tokio::test]
    async fn list_when_channel_id_is_some() {
        struct DummyVideoRepository {}
        #[async_trait]
        impl VideoRepository for DummyVideoRepository {
            async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Video>> {
                Err(anyhow!("not call"))
            }
            async fn list_by_channel(
                &self,
                channel_id: String,
                limit: u32,
                offset: u32,
            ) -> Result<Vec<Video>> {
                Ok(vec![Video::new(
                    "foo".to_string(),
                    "もちもちしてきた".to_string(),
                    Url::try_from("https://example.com").unwrap(),
                    Url::try_from("https://example.com").unwrap(),
                )])
            }
            async fn create(&self, _video: DraftVideo) -> Result<()> {
                Err(anyhow!("not call"))
            }
        }
        let service = VideoService {
            video_repository: std::sync::Arc::new(DummyVideoRepository {}),
        };
        let params = ListVideoParameter {
            limit: 15,
            offset: 0,
            channel_id: Some("foo".to_string()),
        };
        assert_eq!(service.list(params).await.is_ok(), true);
    }
}
