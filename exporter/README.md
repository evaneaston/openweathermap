# openweathermap_exporter

A rust binary to that will poll weather readings for multiple locations and publish the metrics in prometheus exposition format.

This uses [openweathermap_client](https://crates.io/crates/openweathermap_client) to query weather from the API.

## Docs

- [github](https://github.com/evaneaston/openweathermap)
- [docs.rs](https://docs.rs/openweathermap_exporter)
- [crates.io](https://crates.io/crates/openweathermap_exporter)

## Installation

Currently, no binaries or container images are being built. The only way to install it is via:

```
cargo install openweathermap_exporter
```

Building of release binaries and container images is in the works as is addition of separate `/health` and `/metric` endpoints.

## Config File

Create a config file. Start with the the template below.
This file should be named `owm_exporter.yaml` and placed in the working directory from where you plan to run the xporter or in the users home (`~/`) directory.

```yaml
#listen:
#  address: 0.0.0.0  # all Ipv4 addresses is the default
#  port: 9001

owm:
  api_key: # you've got to provide an api key
#  units: metric     # metric is the default
#  language: en      # en is the default

# The exporter doesn't currently warn if the duration of all the calls exceeds the duration
# of `poll_interval_seconds`.  It's up to you to reconfigure so that all readings can be read
# withing the `poll_interval_seconds` timeframe.  This will probably be updated in a future
# release.

#poll_interval_seconds: 60
#max_calls_per_minute: 60

cities:
  - name: Bangkok
    country_code: TH
  - name: New York, NY
    country_code: US

coordinates:
  - lat: -0.829278
    lon: -90.982067

locations:
  - id: 4684888
```

## Running

By default the exporter is pretty quiet. It uses [env_logger](https://crates.io/crates/env_logger) to control the log level.

When first using the exporter, consider running with `info` or `debug` level

```
RUST_LOG=info cargo run
```

Available log levels are `error`, `warn`, `info`, `debug`, `trace`.

## Verify Metrics Are Published

All metrics returned by the free v2.5 API will be exported for scraping. At the moment any route will suffice to load the metrics. If you have not changed the default listen options you can test the your running instance with:

```
curl http://localhost:9001/
```

Metrics all include the unit of the measurement being exported. If you change the setting for `owm.units` in your config file, the names of the metrics might change accordingly.
