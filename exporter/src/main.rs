#![deny(clippy::all, clippy::missing_panics_doc)]
#![warn(
    rustdoc::broken_intra_doc_links,
    clippy::cargo,
    clippy::perf,
    clippy::complexity,
    //clippy::pedantic,
    //missing_docs
)]
// needed because dirs and tokio depend on wasi 0.11.0+wasi-snapshot-preview1
// but metrics-exporter-prometheus depends on 0.10.2+wasi-snapshot-preview1
#![allow(clippy::multiple_crate_versions)]

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
//! ## Config File
//!
//! Create a config file.  Start with the the template below.
//! This file should be named `owm_exporter.yaml` and placed in the working directory from where you plan to run the xporter or in the users home (`~/`) directory.
//!
//! ```yaml
#![doc = include_str!("../owm_exporter-template.yaml")]
//! ```
//!
//! ## Running
//!
//! By default the exporter is pretty quiet.  It uses [env_logger](https://crates.io/crates/env_logger) to control the log level.
//!
//! When first using the exporter, consider running with `info` or `debug` level
//!
//! ```
//! RUST_LOG=info cargo run
//! ```
//!
//! Available log levels are `error`, `warn`, `info`, `debug`, `trace`.
//!
//! ## Verify Metrics Are Published
//!
//! All metrics returned by the free v2.5 API will be exported for scraping.  At the moment any route will suffice to load the metrics.  If you have not changed the default listen options you can test the your running instance with:
//!
//! ```
//! curl http://localhost:9001/
//! ```
//!
//! Metrics all include the unit of the measurement being exported in their name.  If you change the setting for `owm.units` in your config file, the names of the metrics might change accordingly.

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
