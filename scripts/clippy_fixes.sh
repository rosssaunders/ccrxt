#!/bin/bash
# clippy_fixes.sh: Run cargo clippy with automatic fixes for the workspace.
# Usage: ./clippy_fixes.sh
# Requires: cargo, clippy

set -euo pipefail

# Run clippy with --fix to automatically apply suggestions
cargo clippy --fix --allow-dirty --allow-staged --workspace --all-targets -- -D warnings

echo "Clippy fixes applied. Please review the changes before committing."
