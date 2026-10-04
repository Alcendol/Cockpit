#!/usr/bin/env sh
set -eu

cargo test --bin cockpit "$@"
