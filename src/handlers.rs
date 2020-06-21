use std::sync::Arc;
use tokio_postgres::Client;
use warp::{Rejection, Reply};

pub async fn create_channel_handler(pg_client: Arc<Client>) -> Result<impl Reply, Rejection> {
    let db = pg_client.clone();
    let result = pg_client
        .query(
            r#"INSERT INTO channels(channel_id, name, icon_url) VALUES ($1, $2, $3)"#,
            &[&"foo", &"bar", &"https://google.com"],
        )
        .await;
    match result {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::CREATED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
