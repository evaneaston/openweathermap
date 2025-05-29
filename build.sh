#!/usr/bin/env bash
set -euo pipefail

# Approximate the tasks used in the ci workflow

cargo test 
cargo +nightly fmt -- --check
cargo +nightly clippy
cargo +nightly doc
