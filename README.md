# OpenWeatherMap Exporter

This is a workspace repo for two rust crates:

| Crate | Description | Links |
| ------|  ----- | ---- |
| openweathermap_client | A rust library for querying weather readings from OpenWeatherMap's free v2.5 API | [here](./client/README.md) \| [crates.io](https://crates.io/crates/openweathermap_client) \| [docs.rs](https://docs.rs/openweathermap_client) |
| openweathermap_exporter | A rust binary for querying a collection of weather readings for many locations and publishing their values in prometheus exposition format. | [here](./exporter/README.md) \| [crates.io](https://crates.io/crates/openweathermap_exporter) \| [docs.rs](https://docs.rs/openweathermap_exporter) |

Being an initial release, there are a bunch of other ideas planned.  For these see the [TODOs](./TODOs.md).  If you want to contribute to achieve these, reach out.