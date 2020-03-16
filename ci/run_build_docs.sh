#!/bin/bash

. ./features.sh

cargo doc --features "std_web,$NON_CONFLICTING_FEATURES"
