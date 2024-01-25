use core::fmt;

use super::models::{City, CityId, Coord};

pub type QueryParameter = (&'static str, String);
pub type QueryParameters = Vec<QueryParameter>;

/// Abstraction of a query against the OpenWeatherMap API.
pub trait Query: fmt::Debug + fmt::Display + Send + Sync {
    /// Used in logging and included as a label in metrics published by openweathermap_exporter
    fn get_display_name(&self) -> &Option<String>;
    /// Query parameters and values that must be added to API call URL.
    fn query_params(&self) -> QueryParameters;
}

/// Queries weather at a geographic location using `lat={lat},lon={lon}` as described [here](https://openweathermap.org/current#geo).
impl Query for Coord {
    fn get_display_name(&self) -> &Option<String> {
        &self.display_name
    }

    fn query_params(&self) -> QueryParameters {
        vec![("lat", self.lat.to_string()), ("lon", self.lon.to_string())]
    }
}

/// Queries weather at a city using `q={city},{country_code}` as described [here](https://openweathermap.org/current#name).
impl Query for City {
    fn get_display_name(&self) -> &Option<String> {
        &self.display_name
    }

    fn query_params(&self) -> QueryParameters {
        vec![("q", format!("{},{}", self.name, self.country_code))]
    }
}

/// Queries weather at a city by it's openweathermap id as described [here](https://openweathermap.org/current#cityid).
impl Query for CityId {
    fn get_display_name(&self) -> &Option<String> {
        &self.display_name
    }

    fn query_params(&self) -> QueryParameters {
        vec![("id", self.id.to_string())]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{City, CityId, Coord},
        query::QueryParameter,
        Query,
    };

    fn coord_query() -> (Coord, Vec<QueryParameter>) {
        (
            Coord {
                lat: 1.2345,
                lon: 5.6789,
                display_name: None,
            },
            vec![("lat", "1.2345".to_owned()), ("lon", "5.6789".to_owned())],
        )
    }

    fn city_query1() -> (City, Vec<QueryParameter>) {
        (
            City {
                name: "Aripuanã".to_owned(),
                country_code: "BR".to_owned(),
                display_name: None,
            },
            vec![("q", "Aripuanã,BR".to_owned())],
        )
    }

    fn city_query2() -> (City, Vec<QueryParameter>) {
        (
            City {
                name: "Springfield,IL".to_owned(),
                country_code: "US".to_owned(),
                display_name: None,
            },
            vec![("q", "Springfield,IL,US".to_owned())],
        )
    }
    fn city_id_query() -> (CityId, Vec<QueryParameter>) {
        (
            CityId {
                id: 3665202,
                display_name: None,
            },
            vec![("id", "3665202".to_owned())],
        )
    }

    #[test]
    fn test_query_params() {
        let (query, expected) = coord_query();
        assert_eq!(query.query_params(), expected);

        let (query, expected) = city_query1();
        assert_eq!(query.query_params(), expected);

        let (query, expected) = city_query2();
        assert_eq!(query.query_params(), expected);

        let (query, expected) = city_id_query();
        assert_eq!(query.query_params(), expected);
    }

    fn is_sync<T: Sync>(_: &T) -> bool {
        true
    }
    fn is_send<T: Send>(_: &T) -> bool {
        true
    }

    #[test]
    fn ensure_query_stays_send_plus_sync() {
        let (query, _) = coord_query();
        assert!(is_sync(&query));
        assert!(is_send(&query));

        let (query, _) = city_query1();
        assert!(is_sync(&query));
        assert!(is_send(&query));

        let (query, _) = city_query2();
        assert!(is_sync(&query));
        assert!(is_send(&query));

        let (query, _) = city_id_query();
        assert!(is_sync(&query));
        assert!(is_send(&query));
    }
}
