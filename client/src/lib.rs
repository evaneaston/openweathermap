//!
//! ## Get An API Key
//!
//! To obtain an API key, go to [https://openweathermap.org/home/sign_in](https://openweathermap.org/home/sign_in) to
//! sign in or create an account. Once logged in, select your user name from the top-right menu bar and then
//! **My API Keys**. Use the **Create key** form to create a new key.
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
