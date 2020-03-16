#!/bin/bash

. ./features.sh

wasm-pack build --features "std_web,$NON_CONFLICTING_FEATURES"
wasm-pack build --features "web_sys,$NON_CONFLICTING_FEATURES"
