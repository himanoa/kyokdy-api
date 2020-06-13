use thiserror::Error;

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
