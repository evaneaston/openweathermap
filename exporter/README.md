# openweathermap_exporter

A rust binary to that will poll weather readings for multiple locations and publish the metrics in prometheus exposition format.

## Docs

* [github](https://github.com/evaneaston/openweathermap)
* [docs.rs](https://docs.rs/openweathermap_exporter)
* [crates.io](https://crates.io/crates/openweathermap_exporter)

## Installation

Currently, no binaries or container images are being built.  The only way to install it is via

```
cargo install openweathermap_exporter
````

## Config File

See the [docs.rs](https://docs.rs/openweathermap_exporter) site for an example of an `owm_exporter.yaml` config file to configure the exporter.  Alternatively, use the [owm_exporter-template.yaml](../owm_exporter-template.yaml) file inthe git repo as a starting point for creating your own config file.


## Running

Once you have a valid `owm_exporter.yaml` or `owm_exporter.json` file in the `CWD` or `~/` directories, you can run with the the logging level specified in the `RUST_LOG` environment variable.

```
RUST_LOG=info cargo run
```

Control of the loggin level is implemented via the [env_logger](https://crates.io/crates/env_logger) crate). Valid log levels are

* `error`
* `warn`
* `info`
* `debug`
* `trace`

If the `RUST_LOG` environment variable is not defined.  The process will be pretty quiet unless the config file is malformed.

# Metrics

All metrics returned by the free v2.5 API will be exported for scraping.  At the moment any route will suffice to load the metrics.  If you have not changed the default listen options you can test the your running instance with

```
curl http://localhost:9001/
```

Metrics all include the unit of the measurement being exported.  If you change the setting for `owm.units` in your config file, the names of the metrics might change accordingly.
