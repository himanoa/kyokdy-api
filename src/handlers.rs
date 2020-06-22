use std::sync::Arc;
use tokio_postgres::Client;
use warp::{Rejection, Reply};
use crate::IApplication;
use crate::domain::channel::repository::ChannelRepository;

pub async fn create_channel_handler(application: impl IApplication + 'static ) -> Result<impl Reply, Rejection> {
    let repo = application.channel_repository();
    repo.find_by_id("asd");

    Ok(warp::reply::with_status(warp::reply::json(&()), warp::http::StatusCode::CREATED))
}
