use log::error;
use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};
use std::{
    env::VarError,
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
    time::Duration,
};

use openweathermap_client::{
    models::{City, CityId, Coord},
    ClientOptions, Query,
};

use crate::ExporterError;

#[derive(Debug, Deserialize)]
pub struct ListenOptions {
    #[serde(default = "ListenOptions::default_listen_address")]
    pub address: IpAddr,
    #[serde(default = "ListenOptions::default_listen_port")]
    pub port: u16,
}

impl Default for ListenOptions {
    fn default() -> Self {
        Self {
            address: ListenOptions::default_listen_address(),
            port: ListenOptions::default_listen_port(),
        }
    }
}

impl ListenOptions {
    // IP address in environment variable LISTEN_ADDRESS if present or localhost
    fn default_listen_address() -> IpAddr {
        let env_var = "LISTEN_ADDRESS";
        let default_address = || IpAddr::V4(Ipv4Addr::LOCALHOST);

        // TODO fix panic
        match std::env::var(env_var) {
            Ok(address_string) => {
                if address_string.trim().is_empty() {
                    default_address()
                } else {
                    address_string.parse::<IpAddr>().unwrap_or_else(|e| {
                        panic!("Unable to parse environment variable {env_var} {address_string} as a ipv4 address. {e}")
                    })
                }
            }
            Err(VarError::NotPresent) => default_address(),
            Err(VarError::NotUnicode(e)) => panic!("Unable to load environment variable {env_var}. {e:?}"),
        }
    }

    // port  in environment variable LISTEN_PORT if present or 9001
    fn default_listen_port() -> u16 {
        let env_var = "LISTEN_PORT";
        let default_port = 9001;

        // TODO fix panic
        match std::env::var(env_var) {
            Ok(port_string) => {
                if port_string.trim().is_empty() {
                    default_port
                } else {
                    port_string.parse::<u16>().unwrap_or_else(|e| {
                        panic!("Unable to parse environment variable {env_var} {port_string} as a 16-bit integer. {e}")
                    })
                }
            }
            Err(VarError::NotPresent) => default_port,
            Err(VarError::NotUnicode(e)) => panic!("Unable to load environment variable {env_var}. {e:?}"),
        }
    }
}

/// Configuration for the exporter
#[serde_as] // must keep this first
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExporterConfig {
    /// Controls the http server for the exporter
    #[serde(default = "ListenOptions::default")]
    pub listen: ListenOptions,

    /// Configures the [`openweathermap_client::Client`] used by the exporter.
    #[serde(default = "ClientOptions::default")]
    pub owm: ClientOptions,

    /// How frequently to query location weather.
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "ExporterConfig::default_poll_interval")]
    #[serde(rename(deserialize = "poll_interval_seconds"))]
    pub poll_interval: Duration,

    /// Maximum # of calls allow per minute.  Must be > 0.
    #[serde(default = "ExporterConfig::default_max_calls_per_minute")]
    pub max_calls_per_minute: u16,

    /// The [Cities](City) to query weather for.
    #[serde(default = "Vec::new")]
    pub cities: Vec<City>,

    /// The [Coordinates](Coord) to query weather for.
    #[serde(default = "Vec::new")]
    pub coordinates: Vec<Coord>,

    /// The [`CityId`]s to query weather for.
    #[serde(default = "Vec::new")]
    pub locations: Vec<CityId>,
}

impl ExporterConfig {
    fn default_poll_interval() -> Duration {
        Duration::from_secs(60)
    }

    /// Defaults to 60
    pub fn default_max_calls_per_minute() -> u16 {
        60
    }

    /// Loads a configuration from a yaml or json file.
    ///
    /// Searches for the first file from the following list.  Once found stops, and loads it.
    ///
    ///  * `./owm_exporter.yaml`
    ///  * `./owm_exporter.yml`
    ///  * `./owm_exporter.json`
    ///  * `~/owm_exporter.yaml`
    ///  * `~/owm_exporter.yml`
    ///  * `~/owm_exporter.json`
    ///
    /// # Config File Format
    ///
    /// There is a config file template in the git repo and an example of the template in the crate root.
    ///
    /// # Example Usage:
    ///
    /// ```rust
    /// let config = Config::load();
    /// ```
    /// # Errors
    /// If the file is missing, cannot be read, contains a syntax error contains invalid values.
    ///
    pub fn load() -> Result<ExporterConfig, ExporterError> {
        let path = Self::find_config_file()?;
        let contents = read_from_path(&path)?;
        let parsed: ExporterConfig = Self::parse(&path, &contents)?;
        parsed.validate()?;
        Ok(parsed)
    }

    fn parse(path: &Path, contents: &str) -> Result<ExporterConfig, ExporterError> {
        match serde_yaml::from_str::<ExporterConfig>(contents) {
            Ok(config) => Ok(config),
            Err(e) => {
                error!("Error {e:?}");
                Err(ExporterError::ConfigFormatError {
                    path: path.to_string_lossy().to_string(),
                    error: e,
                })
            }
        }
    }

    fn find_config_file() -> Result<PathBuf, ExporterError> {
        let candidates = ["owm_exporter.yaml", "owm_exporter.yml", "owm_exporter.json"];

        let candidate_files: Vec<PathBuf> = [std::env::current_dir().ok(), dirs::home_dir()]
            .iter()
            .flatten()
            .flat_map(|pb| candidates.iter().map(|file| pb.as_path().join(*file)))
            .collect();

        log::debug!("candidate files {candidate_files:?}");

        match candidate_files.iter().find(|f| f.exists() && f.is_file()) {
            Some(pb) => Ok(pb.clone()),
            None => Err(ExporterError::ConfigNotFound {
                message: format!(
                    "Could not locate any of the following config files {}.",
                    join_paths(&candidate_files, ", ")
                ),
            }),
        }
    }

    pub(crate) fn validate(&self) -> Result<(), ExporterError> {
        if self.cities.len() + self.coordinates.len() + self.locations.len() == 0 {
            return Err(ExporterError::ConfigValidationError {
                message: "No cities or coordinates or locations were specified in the config".to_string(),
                error: None,
            });
        }

        if self.max_calls_per_minute == 0 {
            return Err(ExporterError::ConfigValidationError {
                message: "max_calls_per_minute must > 0".to_string(),
                error: None,
            });
        }

        match self.owm.validate() {
            Ok(()) => Ok(()),
            Err(e) => Err(ExporterError::ConfigValidationError {
                message: "Owm Client Validation error".to_string(),
                error: Some(e),
            }),
        }
    }

    // fn to_dyn_query<Q> (input: Vec<&dyn Q>) -> Map<Iter<Q>, |&Q| -> &dyn Query>
    //     where Q: Query {
    //     input.iter().map(|value| value as &dyn Query)
    // }

    pub(crate) fn query_iterator(&self) -> impl Iterator<Item = &dyn Query> {
        let cities = self.cities.iter().map(|c| c as &dyn Query);
        let coordinates = self.coordinates.iter().map(|c| c as &dyn Query);
        let locations = self.locations.iter().map(|c| c as &dyn Query);
        cities.chain(coordinates).chain(locations)
    }
}

fn read_from_path(path: &PathBuf) -> Result<String, ExporterError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(ExporterError::ConfigReadError {
            path: path.to_string_lossy().to_string(),
            error: e,
        }),
    }
}

fn join_paths(pbs: &[PathBuf], separator: &str) -> String {
    pbs.iter()
        .map(|pb| pb.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(separator)
}
