use crate::domain::channel::model::DraftChannel;
use crate::handlers::{create_channel_handler, list_video_handler};
use crate::IApplication;
use log::error;
use serde::{Serialize, Deserialize};
use std::convert::Infallible;
use std::sync::Arc;
use warp::{
    filters::body::BodyDeserializeError, http::StatusCode, reply, Filter, Rejection, Reply,
};

#[derive(Serialize)]
struct ErrorMessage {
    description: String,
}

#[derive(Deserialize)]
struct Pagination {
    limit: i64,
    offset: i64
}

pub fn routes(
    application: Arc<dyn IApplication + Send + Sync>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    create_channel(application.clone())
        .or(list_video(application.clone()))
        .recover(|e| handle_error(e))
}

fn create_channel(
    application: Arc<dyn IApplication + Send + Sync>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("channel")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |body| create_channel_handler(application.clone(), body))
}

fn list_video(
    application: Arc<dyn IApplication + Send + Sync>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("videos")
        .and(warp::get())
        .and(warp::query())
        .and_then(move |p: Pagination| list_video_handler(application.clone(), p.limit, p.offset))
}

async fn handle_error(e: Rejection) -> Result<impl Reply, Rejection> {
    let code;
    let error_message = if e.is_not_found() {
        code = StatusCode::NOT_FOUND;
        ErrorMessage {
            description: "Not found endpoint".to_string(),
        }
    } else if let Some(error) = e.find::<BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        ErrorMessage {
            description: error.to_string(),
        }
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        error!("Internal server error {:?}", e);
        ErrorMessage {
            description: "Internal server error".to_string(),
        }
    };
    Ok(warp::reply::with_status(reply::json(&error_message), code))
}
