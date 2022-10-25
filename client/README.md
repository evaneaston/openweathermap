# openweathermap_client

A rust library that provides a client to query weather readings from OpenWeatherMap's free [version 2.5 weather API](https://openweathermap.org/current).

There is a corresponding binary, [openweathermap_exporter](https://crates.io/crates/openweathermap_exporter) which uses this library to export weather readings in prometheus exposition format.

## Docs

* [github](https://github.com/evaneaston/openweathermap)
* [docs.rs](https://docs.rs/openweathermap_client)
* [crates.io](https://crates.io/crates/openweathermap_client)

## Features:

* Queries over https using hyper
* Binds query results into structs using serde
* Supports requesting results in OWM's `Standard`, `Metric`, or `Imperial` unit systems
* Supports requesting that the API translate of city names and weather descriptions into supported languages
* Is panic-free


