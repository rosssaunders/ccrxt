#!/bin/bash

# Build script for CCRXT Web UI

echo "Building CCRXT Web UI..."

# Install trunk if not already installed
if ! command -v trunk &> /dev/null; then
    echo "Installing trunk..."
    cargo install --locked trunk
fi

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Build the project
echo "Building with trunk..."
trunk build --release

# Apply comprehensive extension fixes
echo "Applying Chrome extension fixes..."
./final_fix.sh

# Validate the build
echo "Validating extension build..."
if [[ ! -f "dist/manifest.json" ]]; then
    echo "ERROR: manifest.json missing from dist/"
    exit 1
fi

if [[ ! -f "dist/background.js" ]]; then
    echo "ERROR: background.js missing from dist/"
    exit 1
fi

if [[ ! -f "dist/index.html" ]]; then
    echo "ERROR: index.html missing from dist/"
    exit 1
fi

echo "✅ Build complete! Extension ready in dist/ directory"
echo "📂 Load the extension from: $(pwd)/dist"