#!/bin/bash
# Generate PyO3 bindings using rustdoc JSON output

set -e

echo "Generating rustdoc JSON output..."
cd venues
cargo doc --no-deps --output-format json --target-dir ../target-doc

echo "Processing rustdoc JSON to generate PyO3 bindings..."
python3 ../tools/rustdoc_to_pyo3.py ../target-doc/doc/venues.json ../python-bindings/src/generated

echo "Done!"
