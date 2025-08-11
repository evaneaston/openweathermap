use log::{debug, error, info};
use metrics::{describe_gauge, describe_histogram, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, SystemTimeError};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration, Interval, MissedTickBehavior};

use openweathermap_client::models::CurrentWeather;
use openweathermap_client::{Client, Query};

#[allow(clippy::wildcard_imports)]
use crate::metric_metadata::*;

use crate::ExporterConfig;
use crate::ExporterError;

/// An exporter will indefinitely query weather for a collection of cities, coordinates or city ids and publish the weather details in prometheus exposition format.
pub struct Exporter {
    config: ExporterConfig,
    client: Client,
    rate_limiter: Arc<Mutex<Interval>>,
}

impl Exporter {
    /// Creates a new exporter for the provided [`ExporterConfig`].  Will fail if
    /// [`openweathermap_client::Client::new`] fails or if there are no cities, coordinates
    /// or locations specified.
    ///
    /// # Errors
    /// If there is a problem configuring the exporter.
    pub fn new(config: ExporterConfig) -> Result<Exporter, ExporterError> {
        config.validate()?;

        let rate_limit_duration_millis = (60000_u64 * 1000_u64 / u64::from(config.max_calls_per_minute)) / 1000_u64;
        let mut rate_limiter = interval(Duration::from_millis(rate_limit_duration_millis));
        rate_limiter.set_missed_tick_behavior(MissedTickBehavior::Delay);

        Ok(Exporter {
            client: Client::new(config.owm.clone())?,
            config,
            rate_limiter: Arc::new(Mutex::new(rate_limiter)),
        })
    }

    /// Starts the exporter and the polling loop. It will return an error if the
    /// http server fails to start. But once started, should never return.  Any failing
    /// API calls will simply be logged with the [`log::Level::Error`] log level.
    ///
    /// # Errors
    /// If the exporter cannot be initialized.
    pub async fn run(&mut self) -> Result<(), ExporterError> {
        info!("Starting");
        info!("config={:?}", self.config);

        self.init_prometheus_exporter()?;
        self.reading_loop().await
    }

    async fn reading_loop(&self) -> ! {
        info!("Starting reading loop");

        let mut poll_interval = interval(self.config.poll_interval);
        poll_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
        poll_interval.reset(); // we want to start the first query immediately

        loop {
            for query in self.config.query_iterator() {
                self.slow_down().await;

                info!("Getting weather for {query:?}");

                let start = SystemTime::now();
                let reading = self.client.fetch_weather(query).await;
                let call_duration = &SystemTime::now().duration_since(start);

                update_call_time_metrics(call_duration);

                match reading {
                    Ok(reading) => self.update_metrics_for_successful_query(query, &reading),
                    Err(e) => {
                        self.update_metrics_for_failed_query(query);
                        error!("Error reading weather for {query:?}. Error: {e:?}");
                    }
                }
            }

            poll_interval.tick().await;
        }
    }

    async fn slow_down(&self) {
        self.rate_limiter.lock().await.tick().await;
    }

    fn init_prometheus_exporter(&self) -> Result<(), ExporterError> {
        let listen_address = self.config.listen.address;
        let port = self.config.listen.port;

        let builder = PrometheusBuilder::new();
        builder
            .idle_timeout(MetricKindMask::ALL, Some(self.config.poll_interval))
            .with_http_listener(SocketAddr::new(listen_address, port))
            .install()?;

        info!("Listening on {listen_address:?}:{port}");

        self.describe_call_metrics();
        self.describe_current_weather_metrics();

        Ok(())
    }

    fn update_metrics_for_successful_query(&self, query: &dyn Query, reading: &CurrentWeather) {
        debug!("updating metrics for successful query {query:?} with reading {reading:?}");

        self.update_query_success_metrics(query, true);

        let labels = labels_for(query, reading);
        self.write_reading_values(reading, &labels);
    }

    fn update_metrics_for_failed_query(&self, query: &dyn Query) {
        debug!("updating metrics for failed query {query:?}");

        self.update_query_success_metrics(query, false);
    }

    #[allow(clippy::unused_self)]
    fn update_query_success_metrics(&self, query: &dyn Query, success: bool) {
        let labels = labels_for_query(query);
        gauge!(OWM_QUERY_SUCCESS.name(), &labels).set(if success { 1. } else { 0. });
    }

    #[allow(clippy::unused_self)]
    fn describe_call_metrics(&self) {
        describe_histogram!(OWM_API_CALL_TIME_HIST.name(), OWM_API_CALL_TIME_HIST.description());
    }

    fn describe_current_weather_metrics(&self) {
        let units = self.config.owm.units;

        for m in [
            OWM_QUERY_SUCCESS,
            OWM_TIMESTAMP_SECONDS,
            owm_temperature(units),
            owm_temperature_feels_like(units),
            OWM_PRESSURE,
            OWM_HUMIDITY_PERCENT,
            owm_wind_speed(units),
            OWM_WIND_DIRECTION,
            owm_wind_gust(units),
            OWM_CLOUDINESS_PERCENT,
            OWM_VISIBILITY,
            OWM_RAIN_1H,
            OWM_RAIN_3H,
            OWM_SNOW_1H,
            OWM_SNOW_3H,
        ] {
            describe_gauge!(m.name(), m.description());
        }
    }

    fn write_reading_values(&self, reading: &CurrentWeather, labels: &Vec<(&'static str, String)>) {
        let units = self.config.owm.units;

        #[allow(clippy::cast_precision_loss)] // precision loss is not going to matter in anyone's lifetime
        gauge!(OWM_TIMESTAMP_SECONDS.name(), labels).set(reading.dt as f64);

        gauge!(owm_temperature(units).name(), labels).set(reading.main.temp);
        gauge!(owm_temperature_feels_like(units).name(), labels).set(reading.main.feels_like);
        gauge!(OWM_PRESSURE.name(), labels).set(reading.main.pressure);
        gauge!(OWM_HUMIDITY_PERCENT.name(), labels).set(reading.main.humidity);
        gauge!(owm_wind_speed(units).name(), labels).set(reading.wind.speed);
        gauge!(OWM_WIND_DIRECTION.name(), labels).set(reading.wind.deg);
        if let Some(gust) = reading.wind.gust {
            gauge!(owm_wind_gust(units).name(), labels).set(gust);
        }
        gauge!(OWM_CLOUDINESS_PERCENT.name(), labels).set(reading.clouds.cloudiness);

        #[allow(clippy::cast_precision_loss)] // precision loss is not going to matter in anyone's lifetime
        if let Some(visibility) = reading.visibility {
            gauge!(OWM_VISIBILITY.name(), labels).set(f64::from(visibility));
        }

        if let Some(pv) = &reading.rain {
            if let Some(mm) = pv.one_hour {
                gauge!(OWM_RAIN_1H.name(), labels).set(mm);
            }
            if let Some(mm) = pv.three_hour {
                gauge!(OWM_RAIN_3H.name(), labels).set(mm);
            }
        }

        if let Some(pv) = &reading.snow {
            if let Some(mm) = pv.one_hour {
                gauge!(OWM_SNOW_1H.name(), labels).set(mm);
            }
            if let Some(mm) = pv.three_hour {
                gauge!(OWM_SNOW_3H.name(), labels).set(mm);
            }
        }
    }
}

fn update_call_time_metrics(call_duration: &Result<Duration, SystemTimeError>) {
    if let Ok(duration) = call_duration {
        #[allow(clippy::cast_precision_loss)]
        // precision loss unimportantwe onlu really coare for values in a range of about 10^8 microseconds
        histogram!(OWM_API_CALL_TIME_HIST.name()).record(duration.as_micros() as f64 / 1000.);
    }
}

fn labels_for_query(query: &dyn Query) -> Vec<(&'static str, String)> {
    let mut labels = query.query_params();
    add_display_name(query, &mut labels);
    labels
}

fn add_display_name(query: &dyn Query, labels: &mut Vec<(&'static str, String)>) {
    if let Some(display_name) = query.get_display_name() {
        labels.push(("display_name", display_name.clone()));
    }
}

fn labels_for(query: &dyn Query, reading: &CurrentWeather) -> Vec<(&'static str, String)> {
    let location = match query.get_display_name() {
        Some(name) => name,
        None => &reading.name,
    };

    let mut labels = vec![("location", location.clone())];

    labels.append(&mut query.query_params());
    add_display_name(query, &mut labels);

    labels.push(("reading_id", reading.id.to_string()));
    labels.push(("reading_lat", reading.coord.lat.to_string()));
    labels.push(("reading_lon", reading.coord.lon.to_string()));
    labels.push(("reading_name", reading.name.clone()));

    labels
}
