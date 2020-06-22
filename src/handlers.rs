use crate::domain::channel::model::DraftChannel;
use crate::domain::channel::repository::ChannelRepository;
use crate::IApplication;
use std::sync::Arc;
use tokio_postgres::Client;
use warp::{Rejection, Reply};

pub async fn create_channel_handler(
    application: impl IApplication + 'static,
    draft_channel: DraftChannel,
) -> Result<impl Reply, Rejection> {
    application.channel_repository().create(draft_channel).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&()),
        warp::http::StatusCode::CREATED,
    ))
}
