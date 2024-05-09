#!/usr/bin/env bash
set -euo pipefail

cargo fmt -- --check
cargo clippy
cargo test 
cargo doc
