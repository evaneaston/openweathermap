#![deny(clippy::all, clippy::missing_panics_doc)]
#![warn(
    rustdoc::broken_intra_doc_links,
    clippy::cargo,
    clippy::perf,
    clippy::complexity,
    clippy::multiple_crate_versions
)]

//!
//! A rust binary to that will poll weather readings for multiple locations and publish the metrics in prometheus exposition format.
//!
//! This uses [openweathermap_client](https://crates.io/crates/openweathermap_client) to query weather from the API.
//!
//! ## Install
//!
//! Currently, no binaries or container images are being built.  The only way to install it is via
//!
//! ```
//! cargo install openweathermap_exporter
//! ````
//!
//! ## Get An API Key
//!
//! To obtain an API key, go to [https://openweathermap.org/home/sign_in](https://openweathermap.org/home/sign_in) to
//! sign in or create an account. Once logged in, select your user name from the top-right menu bar and then
//! **My API Keys**. Use the **Create key** form to create a new key.
//!
//! ## Create A Config File
//!
//! Create a config file. Start with the the template below (also available in source [here](./exporter/owm_exporter-template.yaml) )
//! This file should be named `owm_exporter.yaml` and placed in the working directory from where you plan to run the exporter or in the user's home (`~/` ,  `%USERPROFILE%`) directory.
//!
//! ```yaml
#![doc = include_str!("../owm_exporter-template.yaml")]
//! ```
//!
//! ## Run The Exporter
//!
//!By default the exporter is pretty quiet. It uses [env_logger](https://crates.io/crates/env_logger) to control the log level.
//!
//!When first using the exporter, consider running with `info` or `debug` level
//!
//! ```
//! RUST_LOG=info cargo run
//! ```
//!
//! Available log levels are `error`, `warn`, `info`, `debug`, `trace`.
//!
//! ## Verify Metrics Are Published
//!
//! All metrics returned by the free v2.5 API will be exported for scraping. At the moment any route will suffice to load the metrics. If you have not changed the default listen options you can test the your running instance with:
//!
//! ```bash
//! curl http://localhost:9001/
//! ```
//!
//! ### Metric Names
//!
//! Because metric names [are encouraged](https://prometheus.io/docs/practices/naming/) to contain unit names:
//!
//! > A metric name...
//! >
//! > - ...should have a suffix describing the unit
//!
//! `openweathermap_exporter` metrics all include the unit of the measurement in their name and HELP text. If you change the setting for `owm.units` in your config file, the names of the metrics and their HELP text will change accordingly.
//!

mod config;
mod error;
mod exporter;
mod metric_metadata;

pub use config::ExporterConfig;
pub use error::ExporterError;
pub use exporter::Exporter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ExporterError> {
    env_logger::init();
    let config = ExporterConfig::load()?;
    Exporter::new(config)?.run().await
}
