use http::{uri::InvalidUri, StatusCode};
use std::{
    fmt::{Display, Formatter, Result},
    str::Utf8Error,
};
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("error calling API")]
    ApiCallError(#[from] ApiCallError),

    #[error("error calling API")]
    InvalidOptionsError(#[from] InvalidOptionsError),
}

#[derive(Debug, Error)]
pub enum ApiCallError {
    #[error("error building URI")]
    ErrorFormingUri(#[from] InvalidUri),

    #[error("error building URI")]
    ErrorFormingUrl(#[from] ParseError),

    #[error("unexpected response. Status: {status:?}, Body: {body:?}")]
    InvalidResponsStatus { status: StatusCode, body: String },

    #[error("API call to {url:?} failed. Error: {error:?}")]
    HttpError { error: hyper::Error, url: String },

    #[error("Response body not utf-8 encoded.  Error: {0:?}")]
    ResponseEncodingError(#[from] Utf8Error),

    #[error("Error reading response. Error: {0:?}")]
    ResponseReadError(#[from] hyper::Error),

    #[error("Error parsing response body.  Error: {0:?}")]
    ResponseParseError(#[from] serde_yaml::Error),
}

#[derive(Debug)]
pub struct InvalidOptionsError {
    pub message: String,
}
impl Display for InvalidOptionsError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "invalid client config: {}", self.message)
    }
}
impl std::error::Error for InvalidOptionsError {}
