#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
. "$DIR/features.sh"

wasm-pack build -- --features "$NON_CONFLICTING_FEATURES"
