#!/usr/bin/env sh
set -eu

"$(dirname "$0")/test-unit.sh" "$@"
"$(dirname "$0")/test-integration.sh" "$@"
