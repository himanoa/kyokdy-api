use warp::{Filter, Rejection, Reply, filters};
use crate::Application;
use crate::handlers::{create_channel_handler};
use tokio_postgres::{Client};
use std::sync::Arc;

pub fn routes(
    client: Arc<Client>
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    create_channel(client.clone())
}

fn create_channel(
    client: Arc<Client>
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("channel")
        .and(warp::post())
        .and_then(move || create_channel_handler(client.clone()))
}
