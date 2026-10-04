#!/usr/bin/env sh
set -eu

if [ ! -d tests ]; then
    printf '%s\n' 'No integration test targets found; skipping.'
    exit 0
fi

cargo test --test '*' "$@"
