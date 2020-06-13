pub mod domain;
pub mod infra;
pub mod exception;

pub struct Config<'a> {
    pub db_url: &'a str,
}

pub trait Application
where
    Self: Sized + Clone + Send + Sync,
{
}
