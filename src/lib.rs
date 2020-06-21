pub mod domain;
pub mod exception;
pub mod handlers;
pub mod infra;
pub mod routes;

pub struct Config<'a> {
    pub db_url: &'a str,
}

pub trait Application
where
    Self: Sized + Clone + Send + Sync,
{
}
