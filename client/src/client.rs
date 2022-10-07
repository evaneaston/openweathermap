use http::StatusCode;
use hyper::Uri;
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use log::{debug, trace};
use std::str::FromStr;
use url::Url;

use crate::error::{ApiCallError, InvalidOptionsError};
use crate::options::ClientOptions;
use crate::{models::CurrentWeather, Query};

type HyperClient<C, B> = hyper::client::Client<C, B>;
pub type HttpClient = HyperClient<HttpsConnector<HttpConnector>, Body>;

/// Api docs are here https://openweathermap.org/current
const V25_ENDPOINT: &str = "https://api.openweathermap.org/data/2.5/weather";

//
pub struct Client {
    options: ClientOptions,
    http_client: HttpClient,
}

impl Client {
    /// Create a new client using the supplied options.
    /// Returns an error if it fails because of invalid options.
    pub fn new(options: ClientOptions) -> Result<Client, InvalidOptionsError> {
        options.validate()?;
        Ok(Client {
            options,
            http_client: HyperClient::builder().build::<_, Body>(HttpsConnector::new()),
        })
    }

    /// Fetch the weather for the provided [Query].
    pub async fn fetch_weather(&self, query: &dyn Query) -> Result<CurrentWeather, ApiCallError> {
        let url = self.url_for(query)?;

        let uri = match Uri::from_str(url.as_str()) {
            Ok(u) => Ok(u),
            Err(invalid_uri) => Err(ApiCallError::ErrorFormingUri(invalid_uri)),
        }?;

        debug!(
            "Fetch weather at URL {}",
            self.options.mask_api_key_if_present(url.as_str())
        );

        match self.http_client.get(uri).await {
            Ok(response_body) => {
                debug!("status: {}", response_body.status());
                match response_body.status() {
                    StatusCode::OK => Ok(self.handle_200_response(response_body).await?),
                    sc => Err(self.handle_non_200_response(response_body, &sc).await),
                }
            }
            Err(error) => Err(ApiCallError::HttpError {
                error,
                url: self.options.mask_api_key_if_present(url.as_str()),
            }),
        }
    }

    fn url_for(&self, query: &dyn Query) -> Result<Url, ApiCallError> {
        match Url::parse(V25_ENDPOINT) {
            Ok(mut url) => {
                {
                    let mut query_pairs = url.query_pairs_mut();
                    query_pairs
                        .append_pair("units", &self.options.units.to_string())
                        .append_pair("lang", &self.options.language)
                        .append_pair("appid", &self.options.api_key);
                    for p in query.query_params() {
                        query_pairs.append_pair(p.0, &p.1);
                    }
                }
                Ok(url)
            }
            Err(e) => Err(ApiCallError::ErrorFormingUrl(e)),
        }
    }

    async fn handle_200_response(
        &self,
        response_body: http::Response<hyper::Body>,
    ) -> Result<CurrentWeather, ApiCallError> {
        let body = response_body_as_str(response_body).await?;

        trace!("Response: {}", body);
        match serde_yaml::from_str::<CurrentWeather>(&body) {
            Ok(weather) => Ok(weather),
            Err(e) => Err(ApiCallError::ResponseParseError(e)),
        }
    }

    async fn handle_non_200_response(
        &self,
        response_body: http::Response<hyper::Body>,
        sc: &StatusCode,
    ) -> ApiCallError {
        let rb = match response_body_as_str(response_body).await {
            Ok(rb) => rb,
            Err(error) => format!("Error obtaining response body {:?}", error),
        };
        ApiCallError::InvalidResponsStatus { status: *sc, body: rb }
    }
}

async fn response_body_as_str(response_body: http::Response<hyper::Body>) -> Result<String, ApiCallError> {
    let buf = match hyper::body::to_bytes(response_body).await {
        Ok(ok) => Ok(ok),
        Err(e) => Err(ApiCallError::ResponseReadError(e)),
    }?;
    match std::str::from_utf8(&buf) {
        Ok(str) => Ok(str.to_owned()),
        Err(e) => Err(ApiCallError::ResponseEncodingError(e)),
    }
}
