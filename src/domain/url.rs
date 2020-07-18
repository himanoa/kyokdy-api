use crate::exception::UrlParseFailed;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use url::Url as CrateUrl;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Url(pub String);

impl TryFrom<String> for Url {
    type Error = UrlParseFailed;

    fn try_from(raw: String) -> Result<Url, Self::Error> {
        CrateUrl::parse(raw.clone().as_str()).map_err(|_| UrlParseFailed::new(raw.clone()))?;
        Ok(Url(raw.clone()))
    }
}

impl TryFrom<&str> for Url {
    type Error = UrlParseFailed;

    fn try_from(raw: &str) -> Result<Url, Self::Error> {
        CrateUrl::parse(raw.clone()).map_err(|_| UrlParseFailed::new(raw.clone().to_string()))?;
        Ok(Url(raw.clone().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_string_and_valid_url() {
        assert_eq!(
            Url::try_from("https://google.com".to_string()).unwrap(),
            Url("https://google.com".to_string())
        );
    }
    #[test]
    fn from_string_and_invalid_url() {
        assert_eq!(
            Url::try_from("invalid_url".to_string()).err().unwrap(),
            UrlParseFailed::new("invalid_url".to_string())
        );
    }
    #[test]
    fn from_str_and_valid_url() {
        assert_eq!(
            Url::try_from("https://google.com").unwrap(),
            Url("https://google.com".to_string())
        );
    }
    #[test]
    fn from_str_and_invalid_url() {
        assert_eq!(
            Url::try_from("invalid_url").err().unwrap(),
            UrlParseFailed::new("invalid_url".to_string())
        );
    }
}
