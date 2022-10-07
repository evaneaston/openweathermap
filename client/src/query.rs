use core::fmt;

use super::models::{City, CityId, Coord};

/// Abstraction of a query against the OpenWeatherMap API.
pub trait Query: fmt::Debug + fmt::Display {
    /// Used in logging
    fn get_display_name(&self) -> &Option<String>;
    /// Query parameters and values that must be added to API call URL.
    fn query_params(&self) -> Vec<(&'static str, String)>;
}

/// Queries weather at a geographic location using `lat={lat},lon={lon}` as described [here](https://openweathermap.org/current#geo).
impl Query for Coord {
    fn get_display_name(&self) -> &Option<String> {
        &self.display_name
    }

    fn query_params(&self) -> Vec<(&'static str, String)> {
        vec![("lat", self.lat.to_string()), ("lon", self.lat.to_string())]
    }
}

/// Queries weather at a city using `q={city},{country_code}` as described [here](https://openweathermap.org/current#name).
impl Query for City {
    fn get_display_name(&self) -> &Option<String> {
        &self.display_name
    }

    fn query_params(&self) -> Vec<(&'static str, String)> {
        vec![("q", format!("{},{}", self.name, self.country_code))]
    }
}

/// Queries weather at a city by it's openweathermap id as described [here](https://openweathermap.org/current#cityid).
impl Query for CityId {
    fn get_display_name(&self) -> &Option<String> {
        &self.display_name
    }

    fn query_params(&self) -> Vec<(&'static str, String)> {
        vec![("id", self.id.to_string())]
    }
}
