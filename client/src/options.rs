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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn client_options_default() {
        std::env::set_var("API_KEY", "some value");

        let def = ClientOptions::default();

        assert_eq!(def.api_key, "some value");
        assert_eq!(def.language, "en");
        assert_eq!(def.units, UnitSystem::Metric);
    }

    #[test]
    fn serde_parse() {
        let parsed: ClientOptions = serde_yaml::from_str(
            "\
api_key: abc123
language: \"de\"
units: imperial
",
        )
        .unwrap();

        assert_eq!(parsed.api_key, "abc123");
        assert_eq!(parsed.units, UnitSystem::Imperial);
        assert_eq!(parsed.language, "de");
    }

    #[test]
    fn mask_only_shows_the_first_3_characters_always_followed_by_only_4_stars() {
        assert_eq!(mask("ABCDEFGHIJKLMNOPQRSTUVWZYZ"), "ABC****");
        assert_eq!(mask("ABCDEFGH"), "ABC****");
        assert_eq!(mask("ABCD"), "ABC****");
        assert_eq!(mask("ABC"), "AB****");
        assert_eq!(mask("AB"), "A****");
        assert_eq!(mask("A"), "****");
    }

    #[test]
    fn mask_returns_empty_string_for_empty_input() {
        assert_eq!(mask(""), "");
    }

    #[test]
    fn client_options_debug_masks_api_key() {
        let options = ClientOptions {
            api_key: "PLAINTEXT_API_KEY".to_string(),
            ..ClientOptions::default()
        };
        assert_eq!(options.api_key, "PLAINTEXT_API_KEY");

        let debug = format!("{:?}", options);
        assert!(debug.find("PLAINTEXT") == None);
        assert!(debug.find("PLA****").is_some());
    }

    #[test]
    fn client_options_masked_api_key_masks() {
        let options = ClientOptions {
            api_key: "PLAINTEXT_API_KEY".to_string(),
            ..ClientOptions::default()
        };
        assert_eq!(options.api_key, "PLAINTEXT_API_KEY");
        assert_eq!(options.masked_api_key(), "PLA****");
    }

    #[test]
    fn client_options_mask_api_key_if_present() {
        let options = ClientOptions {
            api_key: "the".to_string(),
            ..ClientOptions::default()
        };
        assert_eq!(
            options.mask_api_key_if_present(
                "I think the quote is, \"It was the best of times, it was the worst of times, ...\""
            ),
            "I think th**** quote is, \"It was th**** best of times, it was th**** worst of times, ...\""
        );
    }
}
