//!
//!
//!
//! # First Obtain An API Key
//!
//! To get an API key, see <a href="#getting-an-openweathermap-api-key">Getting An OpenWeatherMap API Key</a>.
//!
//! ## Example #1
//!
//! ```rust
#![doc = include_str!("../examples/get_weather_summary_in_paris_in_french.rs")]
//! ```
//!
//! ## Example #2
//!
//! ```rust
#![doc = include_str!("../examples/get_multiple_readings.rs")]
//! ```
//!
#![doc = include_str!("../../get_api_key.md")]
#![deny(clippy::all, clippy::missing_panics_doc)]
#![warn(
    rustdoc::broken_intra_doc_links,
    clippy::cargo,
    clippy::perf,
    clippy::complexity,
    //missing_docs,
)]

mod client;
pub mod error;
pub mod models;
mod options;
mod query;

pub use client::Client;
pub use options::ClientOptions;
pub use query::Query;
