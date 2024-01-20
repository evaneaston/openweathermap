use core::fmt;

use super::models::{City, CityId, Coord};

/// Abstraction of a query against the OpenWeatherMap API.
pub trait Query: fmt::Debug + fmt::Display + Send + Sync {
    /// Used in logging and included as a label in metrics published by openweathermap_exporter
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
        vec![("lat", self.lat.to_string()), ("lon", self.lon.to_string())]
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

#[cfg(test)]
mod tests {
    use crate::{
        models::{City, CityId, Coord},
        Query,
    };

    #[test]
    fn coord_includes_lat_and_lon_in_query_params() {
        let q = Coord {
            lat: 1.2345,
            lon: 5.6789,
            display_name: None,
        };
        assert_eq!(
            q.query_params(),
            vec![("lat", "1.2345".to_owned()), ("lon", "5.6789".to_owned())]
        );
    }

    #[test]
    fn city_combines_name_and_country_for_q_value() {
        let q = City {
            name: "Aripuanã".to_owned(),
            country_code: "BR".to_owned(),
            display_name: None,
        };
        assert_eq!(q.query_params(), vec![("q", "Aripuanã,BR".to_owned())]);

        let q = City {
            name: "Springfield,IL".to_owned(),
            country_code: "US".to_owned(),
            display_name: None,
        };
        assert_eq!(q.query_params(), vec![("q", "Springfield,IL,US".to_owned())]);
    }

    #[test]
    fn city_id_includes_city_id_query_params() {
        let q = CityId {
            id: 3665202,
            display_name: None,
        };
        assert_eq!(q.query_params(), vec![("id", "3665202".to_owned())]);
    }
}
