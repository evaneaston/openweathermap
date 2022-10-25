
# TODOs

## Definite Plans

* build and publish a minimal docker image to docker hub
* serve separate /metrics and /health endpoints
    * currently all routes serve metrics
    * I want to change this so health endpoint will if certain bad responses are returning from the OWM API or if all queries are failing, which implies something is wrong such as invalid API key or network routing problems.  This will facilitate container restart in managed environments.
* tests - I've got some pending in a local fork, but don't have any in the public repo, yet
* kubernetes helm chart
* sample grafana dashboards

## Possible Plans

* support for running outside of protected networks
    * https serve support for if you want to run this thing outside a protected network (need to support providing certificate details)
    * some sort of access control, e.g. only allowing callers with some sort of provided creds.  This onlyh really makes sense once https serve support is added
* implement [tracing](https://docs.rs/tracing/latest/tracing).  The client and exporter relies on async calls.  Tracing can make understanding issues easier.

