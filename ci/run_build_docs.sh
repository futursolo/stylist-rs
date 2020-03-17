#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
. "$DIR/features.sh"

cargo doc --features "std_web,$NON_CONFLICTING_FEATURES"
