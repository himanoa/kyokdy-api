use crate::domain::channel::model::DraftChannel;
use crate::domain::channel::repository::ChannelRepository;
use crate::exception::*;
use crate::IApplication;
use warp::{http, reject, reply, Rejection, Reply};

use std::sync::Arc;

pub async fn create_channel_handler(
    application: Arc<dyn IApplication + Send + Sync>,
    draft_channel: DraftChannel,
) -> Result<impl Reply, Rejection> {
    application
        .channel_repository()
        .create(draft_channel)
        .await
        .map_err(|e: anyhow::Error| reject::custom(WarpError(e)))?;

    Ok(reply::with_status(
        reply::json(&()),
        http::StatusCode::CREATED,
    ))
}
