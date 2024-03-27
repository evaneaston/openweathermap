# OpenWeatherMap Client

`openweathermap_client` is a rust library that provides a client for querying OpenWeatherMap's free [version 2.5 weather API](https://openweathermap.org/current).                                                      |

[docs.rs](https://docs.rs/openweathermap_client) | [crates.io](https://crates.io/crates/openweathermap_client)



## Features

- Binds query results into structs derived from [OpenWeatherMap's weather-data docs](https://openweathermap.org/weather-data) using [serde](https://crates.io/crates/serde).
- Supports requesting results in OWM's `Standard`, `Metric`, or `Imperial` unit systems.
- Supports requesting that the API translate of city names and weather descriptions into [supported languages](https://openweathermap.org/current#multi).
- Cross platform. Tested to confirm it runs on Windows, MacOS, and Linux and on many hardware architectures (will be)
- Queries over **https** using [hyper](https://crates.io/crates/hyper) (some existing exporters don't).
  - Doesn't require openssl to be installed, allowing it to be used on weird architectures, because it uses [hyper_rustls](https://crates.io/crates/hyper_rustls).
- Is panic-free.

## Usage

### Get An API Key

To obtain an OpenWeatherMap API Key, see [this section](#getting-an-openweathermap-api-key).

### Add the client to your project

```
cargo add openweathermap_client
```

### Example 1

Get the temperature in °C and description of the weather in Paris right now.

```
use openweathermap_client::models::{City, UnitSystem};
use openweathermap_client::{error::ClientError, Client, ClientOptions};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ClientError> {
    let options = ClientOptions {
        units: UnitSystem::Metric,
        language: "fr".to_string(),
        ..ClientOptions::default() // loads API_KEY env var
    };
    let client = Client::new(options)?;
    let reading = client.fetch_weather(&City::new("Paris", "FR")).await?;

    println!(
        "The temperature and weather in France in French is {}, {}",
        reading.main.temp, reading.weather[0].description
    );
    Ok(())
}
```

### Example 2

Reuse a client to obtain readings at several different locations with different query types.

```
use openweathermap_client::models::{City, CityId, Coord};
use openweathermap_client::{error::ClientError, Client, ClientOptions, Query};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ClientError> {
    // reuse a client for several readings (must be run in an async context)
    let client = Client::new(ClientOptions::default())?; // loads API_KEY env var

    let v: Vec<Box<dyn Query>> = vec![
        Box::new(City::new("Lages", "BR")),
        Box::new(CityId::new(3369157)),
        Box::new(Coord::new(61.1595054, -45.4409551)),
    ];

    for query in v {
        let weather = client.fetch_weather(query.as_ref()).await?;
        println!("The weather for {} is {:?}", query, weather);
    }
    Ok(())
}

```

## Related Crates

See [openweathermap_exporter](https://crates.io/crates/openweathermap_exporter) for a service that allow collecting 
weather data for a multiple locations and sharing this with metrics aggregators by publishing the readings via a prometheus exposition formatted exporter.