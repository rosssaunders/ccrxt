#!/bin/bash
# CCRXT Python Bindings Development Script

set -e

echo "🔧 CCRXT Python Bindings Development"
echo "======================================"

# Step 1: Install required tools
echo "📦 Installing required tools..."
if ! command -v maturin &> /dev/null; then
    echo "Installing maturin..."
    pip install maturin
fi

# Step 2: Generate the bindings using our build script
echo "🏗️  Building the project to generate bindings..."
cd python-bindings
cargo build --release

# Step 3: Build the Python module
echo "🐍 Building Python module..."
maturin develop --release

# Step 4: Test the bindings
echo "🧪 Testing bindings..."
cd ..
python3 tools/test_bindings.py

echo "✅ Development setup complete!"
echo ""
echo "📝 What was generated:"
echo "  - Python bindings in python-bindings/src/generated/"
echo "  - Rust PyO3 wrappers for all venues"
echo "  - Automatic detection based on naming conventions"
echo ""
echo "🚀 Next steps:"
echo "  1. Test the bindings: python3 tools/test_bindings.py"
echo "  2. Create examples: python-bindings/examples/"
echo "  3. Add type stubs: python-bindings/python/ccrxt/stubs/"
echo "  4. Build wheels: maturin build --release"
