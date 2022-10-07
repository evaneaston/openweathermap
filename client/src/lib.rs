//!
//!
//! # Example usage
//!
//! ```rust
//! /// Gets the temperature in °C and description of the weather in Paris right now
//! async fn get_weather_summary_in_paris_in_french() -> Result<(f64, String), ClientError> {
//!     let options = ClientOptions {
//!         api_key: "...".to_string(),
//!         units: UnitSystem::Metric,
//!         language: "fr".to_string(),
//!     };
//!     let mut client = Client::new(options)?;
//!     let reading = client.fetch_weather(&City::new("Paris", "FR")).await?;
//!     Ok((reading.main.temp, reading.main.description))
//! }
//! ```
//!
//! ```rust
//! #[tokio::main]
//! async fn main() -> Result<(), ClientError> {
//!     /// reuse a client for several readings (must be run in an async context)
//!     let mut client = Client::new(ClientOptions::new_default_with_api_key("..."))?;
//!
//!     let v: Vec<Box<dyn Query>> = vec![
//!         Box::new(City::new("Lages", "BR")),
//!         Box:new(CityId::new(3369157)),
//!         Box::new(Coord::new(61.1595054, -45.4409551)),
//!     ];
//!
//!     for query in v {
//!         let weather = client.fetch_weather(query.as_ref()).await?;
//!         println!("The weather for {} is {:?}", query, weather);
//!     }
//!     Ok(())
//! }
//! ```
//!

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
