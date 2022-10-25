//!
//!
//! # Example usage
//!
//! ```rust
#![doc = include_str!("../examples/get_weather_summary_in_paris_in_french.rs")]
//! ```
//!
//! ```rust
#![doc = include_str!("../examples/get_multiple_readings.rs")]
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
