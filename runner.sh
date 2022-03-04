#!/bin/bash

rm -r frames

cargo run --release "$1"

python3 analysis.pyw
