#!/usr/bin/env bash
# tools/rustfmt_nightly.sh
# Run rustfmt using the nightly toolchain with project settings.
# Usage: ./tools/rustfmt_nightly.sh

set -euo pipefail

# Ensure nightly toolchain is installed
if ! rustup toolchain list | grep -q '^nightly'; then
    echo "Installing nightly toolchain..."
    rustup toolchain install nightly
fi

# Ensure rustfmt component is installed for nightly
if ! rustup component list --toolchain nightly | grep -q '^rustfmt.*(installed)'; then
    echo "Adding rustfmt component to nightly..."
    rustup component add rustfmt --toolchain nightly
fi

# Run rustfmt with nightly
exec cargo +nightly fmt "$@"
