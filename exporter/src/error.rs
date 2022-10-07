use openweathermap_client::error::{ApiCallError, ClientError, InvalidOptionsError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExporterError {
    #[error("the metrics exporter failed to initialize")]
    ExporterStartFailure(#[from] metrics_exporter_prometheus::BuildError),

    #[error("config file not found. {message:?}")]
    ConfigNotFound { message: String },

    #[error("error reading config file {path:?}. Error: {error:?}")]
    ConfigReadError { path: String, error: std::io::Error },

    #[error("error parsing config file {path:?}. Error: {error:?}")]
    ConfigFormatError { path: String, error: serde_yaml::Error },

    #[error("Invalid configuration: {message:?}")]
    ConfigValidationError {
        message: String,
        error: Option<InvalidOptionsError>,
    },

    #[error("The OpenWeatherMapClient experienced an error")]
    ApiCallError(#[from] ApiCallError),

    #[error("The OpenWeatherMapClient experienced an error")]
    ClientError(#[from] ClientError),
}

impl From<InvalidOptionsError> for ExporterError {
    fn from(error: InvalidOptionsError) -> Self {
        ExporterError::ConfigValidationError {
            message: "invalid client options".to_string(),
            error: Some(error),
        }
    }
}
