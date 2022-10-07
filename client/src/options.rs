use serde::Deserialize;
use std::{
    cmp::{max, min},
    fmt,
};

use crate::error::InvalidOptionsError;

use super::models::UnitSystem;

/// Options to configure the [Client](super::client::Client).
#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ClientOptions {
    /// An api key required to call the API.
    #[serde(default = "String::new")]
    pub api_key: String,

    /// See <https://openweathermap.org/current#multi> for currently supported languages.  This library will not attempt to validate the language passed in.
    #[serde(default = "ClientOptions::default_language")]
    pub language: String,

    /// Controls the units of certain metrics returned from the API.
    #[serde(default = "ClientOptions::default_units")]
    pub units: UnitSystem,
}

impl ClientOptions {
    /// Useful for creating options with an API Key specified in the environment variable API_KEY and with defaults of "en", Metric, 1 call/sercond.
    pub fn default() -> ClientOptions {
        ClientOptions {
            api_key: Self::default_api_key(),
            language: Self::default_language(),
            units: Self::default_units(),
        }
    }

    /// Create an instance with all defaults and the specified api_key.
    pub fn new_default_with_api_key(api_key: &str) -> ClientOptions {
        ClientOptions {
            api_key: api_key.to_string(),
            language: Self::default_language(),
            units: Self::default_units(),
        }
    }

    // Defaults to the API_KEY environment variable or blank if the env var is not defined.
    pub fn default_api_key() -> String {
        match std::env::var("API_KEY") {
            Ok(api_key) => api_key,
            Err(_) => "".to_string(),
        }
    }

    /// Defaults to "en"
    pub fn default_language() -> String {
        "en".to_string()
    }

    /// Defaults to [UnitSystem::Metric]
    pub fn default_units() -> UnitSystem {
        UnitSystem::Metric
    }

    /// Returns the API key with most of the characters masked out.
    pub fn masked_api_key(&self) -> String {
        mask(&self.api_key)
    }

    /// Ensures an api_key is provided
    pub fn validate(&self) -> Result<(), InvalidOptionsError> {
        if self.api_key.is_empty() {
            return Err(InvalidOptionsError {
                message: "api_key must be non-blank".to_string(),
            });
        }

        Ok(())
    }

    /// Take an arbitrary string that might have the self.api_key in it and returns a string with that all occurrences of the key masked.
    pub fn mask_api_key_if_present(&self, any_string: &str) -> String {
        any_string.replace(&self.api_key, &self.masked_api_key())
    }
}

fn mask(s: &str) -> String {
    let mut masked: String = s.to_string();
    if !s.is_empty() {
        let range = max(0, min(masked.len() - 1, 3))..;
        masked.replace_range(range, "****");
    }
    masked
}

impl fmt::Debug for ClientOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{ api_key: \"{}\", language: \"{}\", units: {} }}",
            mask(&self.api_key),
            self.language,
            self.units
        )
    }
}
