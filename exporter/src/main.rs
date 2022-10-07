//!
//! # Getting Started
//!
//! Create a config file.  Start with the the template below or use [the template from the git repo](../../owm_exporter-template.yaml).
//!
//! ```bash
//! RUST_LOG=info cargo run
//! ```
//!
//! ## Template owm_exporter.yaml
//!
//! Note that defaults are commented out.
//!
//! ```yaml
//! #listen:
//! #  address: 0.0.0.0
//! #  port: 9001
//!
//! owm:
//!   api_key:
//! #  units: metric # Valid units are standard, metric, imperial
//! #  language: en
//!
//! # The exporter doesn't currently warn if the duration of all the calls exceeds the duration
//! # of `poll_interval_seconds`.  It's up to you to reconfigure so that all readings can be read
//! # withing the `poll_interval_seconds` timeframe.  This will probably be updated in a future
//! # release.
//!
//! #poll_interval_seconds: 60
//! #max_calls_per_minute: 60
//!
//! # You can query by [City], [Coord]inates, or [CityId] (aka locations).
//! # All queries have an optional `display_name` property that, if specified, will be shown in metric's `location` label instead of the name that the OWM API returns for the location.
//!
//! cities:
//!   - name: Bangkok
//!     country_code: TH
//!   - name: New York, NY
//!     country_code: US
//!
//! coordinates:
//!   - lat: -0.829278
//!     lon: -90.982067
//!     # display name is optional
//!     display_name: Somewhere in Equador
//!
//! # If you know the OWM city id ([see here](https://openweathermap.org/city)), it's generally better to tell the exporter to query it by id rather than by name+country code.
//! locations:
//!   - id: 4684888
//!     display_name: Dallas, TX
//!     
//!```
//!

#![deny(clippy::all, clippy::missing_panics_doc)]
#![warn(
    rustdoc::broken_intra_doc_links,
    clippy::cargo,
    clippy::perf,
    clippy::complexity,
    //clippy::pedantic,
    //missing_docs
)]

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
