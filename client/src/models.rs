use std::fmt::Display;

use serde::Deserialize;

/// According to [OpenWeatherMap API Docs](https://openweathermap.org/weather-data) and experiments, the return
/// types for each unit are:
///
/// | Measurement Type           | Metric                 | Standard               | Imperial               |
/// | -------------------------- | ---------------------- | ---------------------- | ---------------------- |
/// | Temperature                | °C                     | °K                     | °F                     |
/// | Precipitation Accumulation | mm                     | mm                     | *mm*                   |
/// | Direction                  | meteorological degrees | meteorological degrees | meteorological degrees |
/// | Pressure                   | hectopascal (hPa)      | hectopascal (hPa)      | hectopascal (hPa)      |
/// | Speed                      | meters/second          | meters/second          | miles/hour             |
/// | Visibility                 | meters                 | meters                 | *meters*               |
/// | Percent                    | percent (0-100)        | percent (0-100)        | percent (0-100)        |
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum UnitSystem {
    /// What you might think.
    Metric,
    /// Standard is really the same Metric but with temperatures in °K instead of °C
    Standard,
    /// OWM implements Imperial as Metric with °F and miles/hour, but still uses *meters* for visibility and *mm* for precip ¯\\\_(ツ)_/¯
    Imperial,
}

impl Display for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnitSystem::Standard => write!(f, "standard"),
            UnitSystem::Metric => write!(f, "metric"),
            UnitSystem::Imperial => write!(f, "imperial"),
        }
    }
}

/// Used to query the weather in a particular city via a geocoding lookup
#[derive(Debug, Deserialize, Clone)]
pub struct City {
    /// in many countries format with {cityname},{subdivision} where "subdivision" is a state or province and is specified by the last part of a [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2)
    pub name: String,

    /// An [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) 2-character country code
    pub country_code: String,

    /// When available will be rendered by [Display] instead of the `name`, `country_code`.
    pub display_name: Option<String>,
}
impl City {
    /// Create an instance with just `name` and `country_code`
    pub fn new(name: &str, country_code: &str) -> City {
        City {
            name: name.to_string(),
            country_code: country_code.to_string(),
            display_name: None,
        }
    }
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.display_name {
            Some(display_name) => write!(f, "{display_name}"),
            None => write!(f, "{}, {}", self.name, self.country_code),
        }
    }
}

/// Used to query the weather in a particular city using openweathermap's city id
#[derive(Debug, Deserialize, Clone)]
pub struct CityId {
    /// The [OpenWeatherMap city id](https://openweathermap.org/current#cityid).
    pub id: u32,
    /// When available will be rendered by [Display] instead of the city [CityId#id].
    pub display_name: Option<String>,
}
impl CityId {
    /// Create and instance from just the city's id
    pub fn new(id: u32) -> CityId {
        CityId { id, display_name: None }
    }
}
impl Display for CityId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.display_name {
            Some(display_name) => write!(f, "{display_name}"),
            None => write!(f, "{}", self.id),
        }
    }
}

/// Used to query weather at a latitude and logitude.
#[derive(Debug, Deserialize, Clone)]
pub struct Coord {
    /// City geo location, latitude
    pub lat: f64,
    /// City geo location, longitude
    pub lon: f64,
    /// When available will be rendered by [Display] instead of the lat and lon.
    pub display_name: Option<String>,
}
impl Coord {
    /// Create an instance from just lat and lon
    pub fn new(lat: f64, lon: f64) -> Coord {
        Coord {
            lat,
            lon,
            display_name: None,
        }
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.display_name {
            Some(display_name) => write!(f, "{display_name}"),
            None => write!(f, "lat={}, lon={}", self.lat, self.lon),
        }
    }
}

/// Main structure for responses from the free `OpenWeatherMap` API
///
/// See their API response documenation [here](https://openweathermap.org/current#fields_json).
#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    /// City geo location, longitude
    pub coord: Coord,
    /// Seems to generally be a collection of one
    pub weather: Vec<Weather>,
    /// `OpenWeatherMap` documents as "Internal parameter"
    pub base: String,

    /// Main readings that are usually present in responses.  See [Main].
    pub main: Main,

    /// Visibility, meter. The maximum value of the visibility is 10km
    pub visibility: Option<u16>,
    /// See [Wind]
    pub wind: Wind,
    /// See [Clouds]
    pub clouds: Clouds,
    /// Recent rain volume
    pub rain: Option<PrecipVolume>,
    /// Recent snow volume
    pub snow: Option<PrecipVolume>,
    /// Time of data calculation, unix, UTC (in seconds)
    pub dt: i64,
    /// See [Sys]
    pub sys: Sys,
    /// Shift in seconds from UTC
    pub timezone: i64,
    /// City ID
    pub id: u64,
    /// City name
    pub name: String,
    /// Internal parameter
    pub cod: u64,
}

/// Weather condition description
#[derive(Debug, Deserialize)]
pub struct Weather {
    /// Weather condition id
    pub id: u64,
    /// Group of weather parameters (Rain, Snow, Extreme etc.)
    pub main: String,
    /// Weather condition within the group. You can get the output in your language.
    pub description: String,
    /// Weather icon id
    pub icon: String,
}

/// Detailed weather report
#[derive(Debug, Deserialize)]
pub struct Main {
    /// Temperature. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp: f64,
    /// Temperature. This temperature parameter accounts for the human perception of weather. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub feels_like: f64,
    /// Minimum temperature at the moment. This is minimal currently observed temperature (within large megalopolises and urban areas). Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp_min: f64,
    /// Maximum temperature at the moment. This is maximal currently observed temperature (within large megalopolises and urban areas). Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp_max: f64,
    /// Atmospheric pressure (on the sea level, if there is no `sea_level` or `grnd_level` data), hPa
    pub pressure: f64,
    /// Atmospheric pressure on the sea level, hPa
    pub sea_level: Option<f64>,
    /// Atmospheric pressure on the ground level, hPa
    pub grnd_level: Option<f64>,
    /// Humidity, %
    pub humidity: f64,
}

/// Detailed wind report
#[derive(Debug, Deserialize)]
pub struct Wind {
    /// Wind speed. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour.
    pub speed: f64,
    /// Wind direction, degrees (meteorological)
    pub deg: f64,
    /// Wind gust. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour
    pub gust: Option<f64>,
}

/// Cloudiness readings
#[derive(Debug, Deserialize)]
pub struct Clouds {
    /// Cloudiness, %
    #[serde(rename(deserialize = "all"))]
    pub cloudiness: f64,
}

/// 1- and 3- hour precipitation amounts.  Used for both rain and snow.
#[derive(Debug, Deserialize)]
pub struct PrecipVolume {
    /// Volume for the last 1 hour, mm
    #[serde(rename(deserialize = "1h"))]
    pub one_hour: Option<f64>,

    /// Volume for the last 3 hours, mm
    #[serde(rename(deserialize = "3h"))]
    pub three_hour: Option<f64>,
}

/// Additional information
#[derive(Debug, Deserialize)]
pub struct Sys {
    /// Internal parameter    
    pub type_: Option<u64>,
    /// Internal parameter
    pub id: Option<u64>,
    /// Internal parameter
    pub message: Option<f64>,
    /// Country code (GB, JP etc.)
    pub country: Option<String>,
    /// Sunrise time, unix, UTC
    pub sunrise: i64,
    /// Sunset time, unix, UTC
    pub sunset: i64,
}
