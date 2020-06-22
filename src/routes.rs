use crate::domain::channel::model::DraftChannel;
use crate::handlers::create_channel_handler;
use crate::IApplication;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    application: impl IApplication + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    create_channel(application)
}

fn create_channel(
    application: impl IApplication + 'static,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("channel")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |body| {
            create_channel_handler(application.clone(), body)
        })
}
