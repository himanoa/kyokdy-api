pub mod domain;
pub mod exception;
pub mod handlers;
pub mod infra;
pub mod routes;
pub mod initializer;

use std::sync::Arc;

pub struct Config<'a> {
    pub db_url: &'a str,
}

#[derive(Clone)]
pub struct Application {
    pub channel_repository: Arc<dyn domain::channel::repository::ChannelRepository>,
}

