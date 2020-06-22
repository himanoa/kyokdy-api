pub mod domain;
pub mod exception;
pub mod handlers;
pub mod infra;
pub mod routes;
pub mod initializer;
pub mod application;

use std::sync::Arc;

pub struct Config<'a> {
    pub db_url: &'a str,
}

pub trait IApplication
where
    Self: Sized + Clone + Send + Sync,
{
    type ChannelRepository: domain::channel::repository::ChannelRepository;

    fn channel_repository(&self) -> Self::ChannelRepository;
}

