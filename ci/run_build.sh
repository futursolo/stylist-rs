#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
. "$DIR/features.sh"

wasm-pack build -- --features "std_web,$NON_CONFLICTING_FEATURES"
wasm-pack build -- --features "web_sys,$NON_CONFLICTING_FEATURES"
