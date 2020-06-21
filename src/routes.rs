use crate::handlers::create_channel_handler;
use crate::Application;
use std::sync::Arc;
use tokio_postgres::Client;
use warp::{filters, Filter, Rejection, Reply};

pub fn routes(client: Arc<Client>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    create_channel(client.clone())
}

fn create_channel(
    client: Arc<Client>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("channel")
        .and(warp::post())
        .and_then(move || create_channel_handler(client.clone()))
}
