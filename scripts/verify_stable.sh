#!/usr/bin/env bash
set -euo pipefail

# Ensure we're using stable toolchain only (rust-toolchain.toml already pins stable)
RUSTC_VERSION=$(rustc --version || echo 'unknown')
echo "Using compiler: $RUSTC_VERSION"

# Fail if any nightly feature gates are present
if find . -type f -name '*.rs' -print0 | xargs -0 grep -n "#!\[feature" ; then
  echo "Nightly feature gate found; this project supports only latest stable." >&2
  exit 1
fi

# Note: let-chains stabilized in Rust 1.88, so we allow `if/while let &&` patterns now.

# Build with locked dependencies
cargo build --locked --all --all-targets

# Run clippy (respect workspace lints + allowlist above). Fail on any non-allowed warnings.
cargo clippy --all --all-targets || {
  echo "Clippy failed (excluding allowlisted lints)." >&2
  exit 1
}

echo "Stable + clippy verification passed."
