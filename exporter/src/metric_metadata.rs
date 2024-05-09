use openweathermap_client::models::UnitSystem;

pub struct MetricMetadata<'a> {
    name: &'a str,
    description: &'a str,
}
impl<'a> MetricMetadata<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn description(&self) -> &'a str {
        self.description
    }
}

const fn new_metric<'a>(name: &'a str, description: &'a str) -> MetricMetadata<'a> {
    MetricMetadata { name, description }
}

pub const OWM_QUERY_SUCCESS: &MetricMetadata = &new_metric(
    "owm_query_success",
    "Whether the most recent query for a location's weather succeeded (0 or 1)",
);

const OWM_TEMPERATURE_DEGREES_CELSIUS: &MetricMetadata =
    &new_metric("owm_temperature_degrees_celsius", "Temperature in °C");
const OWM_TEMPERATURE_DEGREES_FARENHEIGHT: &MetricMetadata =
    &new_metric("owm_temperature_degrees_farenheight", "Temperature in °F");
const OWM_TEMPERATURE_DEGREES_KELVIN: &MetricMetadata =
    &new_metric("owm_temperature_degrees_kelvin", "Temperature in °K");
const OWM_TEMPERATURE_FEELS_LIKE_DEGREES_CELSIUS: &MetricMetadata = &new_metric(
    "owm_temperature_feels_like_degrees_celsius",
    "Perceived temperature in °C",
);
const OWM_TEMPERATURE_FEELS_LIKE_DEGREES_FARENHEIGHT: &MetricMetadata = &new_metric(
    "owm_temperature_feels_like_degrees_farenheight",
    "Perceived temperature in °F",
);
const OWM_TEMPERATURE_FEELS_LIKE_DEGREES_KELVIN: &MetricMetadata = &new_metric(
    "owm_temperature_feels_like_degrees_kelvin",
    "Perceived temperature in °K",
);
const OWM_WIND_GUST_METERS_PER_SECOND: &MetricMetadata =
    &new_metric("owm_wind_gust_meters_per_second", "Wind gust speed in meters/second");
const OWM_WIND_GUST_MILES_PER_HOUR: &MetricMetadata =
    &new_metric("owm_wind_gust_miles_per_hour", "Wind gust speed in miles per hour");
const OWM_WIND_SPEED_METERS_PER_SECOND: &MetricMetadata =
    &new_metric("owm_wind_speed_meters_per_second", "Wind speed in meters/second");
const OWM_WIND_SPEED_MILES_PER_HOUR: &MetricMetadata =
    &new_metric("owm_wind_speed_miles_per_hour", "Wind speed in miles per hour");
pub const OWM_CLOUDINESS_PERCENT: &MetricMetadata = &new_metric("owm_cloudiness_percent", "% cloudiness");
pub const OWM_HUMIDITY_PERCENT: &MetricMetadata = &new_metric("owm_humidity_percent", "% Humidity");
pub const OWM_PRESSURE: &MetricMetadata = &new_metric("owm_pressure_hpa", "Atmospheric pressure in hPa");
pub const OWM_RAIN_1H: &MetricMetadata = &new_metric("owm_rain_1h_mm", "1-hour rain accumulation in mm");
pub const OWM_RAIN_3H: &MetricMetadata = &new_metric("owm_rain_3h_mm", "3-hour rain accumulation in mm");
pub const OWM_SNOW_1H: &MetricMetadata = &new_metric("owm_snow_1h_mm", "1-hour snow accumulation in mm");
pub const OWM_SNOW_3H: &MetricMetadata = &new_metric("owm_snow_3h_mm", "3-hour snow accumulation in mm");
pub const OWM_TIMESTAMP_SECONDS: &MetricMetadata = &new_metric(
    "owm_timestamp_seconds",
    "Timestamp of last reading in seconds since UNIX epoch",
);
pub const OWM_VISIBILITY: &MetricMetadata = &new_metric("owm_visibility_meters", "Visibility in meters, 10000 max");
pub const OWM_WIND_DIRECTION: &MetricMetadata =
    &new_metric("owm_wind_direction_degrees", "Wind direction in degrees (0-360)");

pub fn owm_temperature(units: UnitSystem) -> &'static MetricMetadata<'static> {
    match units {
        UnitSystem::Standard => OWM_TEMPERATURE_DEGREES_KELVIN,
        UnitSystem::Metric => OWM_TEMPERATURE_DEGREES_CELSIUS,
        UnitSystem::Imperial => OWM_TEMPERATURE_DEGREES_FARENHEIGHT,
    }
}

pub fn owm_temperature_feels_like(units: UnitSystem) -> &'static MetricMetadata<'static> {
    match units {
        UnitSystem::Standard => OWM_TEMPERATURE_FEELS_LIKE_DEGREES_KELVIN,
        UnitSystem::Metric => OWM_TEMPERATURE_FEELS_LIKE_DEGREES_CELSIUS,
        UnitSystem::Imperial => OWM_TEMPERATURE_FEELS_LIKE_DEGREES_FARENHEIGHT,
    }
}

pub fn owm_wind_gust(units: UnitSystem) -> &'static MetricMetadata<'static> {
    match units {
        UnitSystem::Imperial => OWM_WIND_GUST_MILES_PER_HOUR,
        _ => OWM_WIND_GUST_METERS_PER_SECOND,
    }
}

pub fn owm_wind_speed(units: UnitSystem) -> &'static MetricMetadata<'static> {
    match units {
        UnitSystem::Imperial => OWM_WIND_SPEED_MILES_PER_HOUR,
        _ => OWM_WIND_SPEED_METERS_PER_SECOND,
    }
}

pub const OWM_API_CALL_TIME_HIST: &MetricMetadata = &new_metric(
    "owm_api_call_time_milliseconds",
    "Histogram of successful call times per location in milliseconds",
);
