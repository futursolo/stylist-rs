#!/usr/bin/env bash

echo "$(rustup default)" | grep -q "stable"
if [ "$?" != "0" ]; then
  # only run checks on stable
  exit 0
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
. "$DIR/features.sh"

set -euxo pipefail
cargo fmt --all -- --check
cargo clippy --features "std_web,$NON_CONFLICTING_FEATURES" -- --deny=warnings
cargo clippy --features "web_sys,$NON_CONFLICTING_FEATURES" -- --deny=warnings