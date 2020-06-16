use thiserror::Error;
use anyhow::Error as AnyHowError;

#[derive(Error, Debug, Eq, PartialEq)]
#[error("{url:?} is invalid url")]
pub struct UrlParseFailed {
    url: String
}

impl UrlParseFailed {
    pub fn new(url: String) -> Self {
        UrlParseFailed { url }
    }
}

#[derive(Error, Debug)]
#[error("Database error Query")]
pub struct DataBaseError {
    error: AnyHowError,
}

impl DataBaseError {
    pub fn new(error: AnyHowError) -> Self {
        DataBaseError { error }
    }
}
