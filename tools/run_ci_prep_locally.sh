#!/usr/bin/env bash
# Run the same checks as CI locally, preparing the tree for commit/push.
# - Formatting with nightly rustfmt (auto-fix)
# - Linting, build checks, and tests
# - Docs build + deadlinks (HTTP checks)
# - Security audits (cargo-audit and cargo-deny)
#
# Usage:
#   tools/run_ci_prep_locally.sh [--fast] [--skip-docs] [--skip-security] [--skip-tests]
#
# Flags:
#   --fast           Skip docs + security (quick iteration: check, fmt-fix, clippy, tests)
#   --skip-docs      Skip docs + deadlinks
#   --skip-security  Skip cargo audit and cargo deny
#   --skip-tests     Skip cargo test
#
# Notes:
# - Requires rustup. Uses +stable for build/clippy/tests, +nightly for rustfmt to honor
#   unstable settings in rustfmt.toml (e.g., unstable_features, group_imports, etc.).
# - Will attempt to install cargo-deadlinks, cargo-audit, and cargo-deny if missing.
set -euo pipefail

CARGO_TERM_COLOR=${CARGO_TERM_COLOR:-always}
export CARGO_TERM_COLOR

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." &>/dev/null && pwd)"
cd "$REPO_ROOT"

# Defaults
DO_DOCS=1
DO_SECURITY=1
DO_TESTS=1

for arg in "$@"; do
  case "$arg" in
    --fast)
      DO_DOCS=0
      DO_SECURITY=0
      ;;
    --skip-docs)
      DO_DOCS=0
      ;;
    --skip-security)
      DO_SECURITY=0
      ;;
    --skip-tests)
      DO_TESTS=0
      ;;
    -h|--help)
      sed -n '1,60p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown option: $arg" >&2
      exit 2
      ;;
  esac
  shift || true
done

banner() {
  echo
  echo "========================================"
  echo "==> $*"
  echo "========================================"
}

die() {
  echo "Error: $*" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "Missing required command: $1"
}

ensure_tool() {
  # ensure_tool <crate> <binary> [version]
  local crate="$1"; shift
  local bin="$1"; shift
  local version="${1:-}"

  if ! command -v "$bin" >/dev/null 2>&1; then
    echo "Installing $crate..."
    if [[ -n "$version" ]]; then
      cargo +stable install "$crate@$version"
    else
      cargo +stable install "$crate"
    fi
  fi
}

# Pre-flight checks
need_cmd cargo
need_cmd rustup

banner "linux-core: check, fmt (nightly:fix), clippy, ${DO_TESTS:+tests}"
# Matches CI linux-core job (except fmt is nightly + fix)
cargo +stable check --workspace --all-features
# Use nightly rustfmt to enable unstable features defined in rustfmt.toml and FIX formatting
"${SCRIPT_DIR}/rustfmt_nightly.sh" --all
cargo +stable clippy --all-targets --all-features -- -D warnings
if [[ "$DO_TESTS" == "1" ]]; then
  cargo +stable test --workspace --all-features
fi

if [[ "$DO_DOCS" == "1" ]]; then
  banner "docs: cargo doc + deadlinks (HTTP)"
  ensure_tool cargo-deadlinks deadlinks
  cargo +stable doc --no-deps
  cargo +stable deadlinks --check-http
else
  echo "Skipping docs + deadlinks"
fi

if [[ "$DO_SECURITY" == "1" ]]; then
  banner "security: cargo-audit + cargo-deny"
  ensure_tool cargo-audit cargo-audit
  ensure_tool cargo-deny cargo-deny
  cargo +stable audit
  cargo +stable deny check
else
  echo "Skipping security checks"
fi

banner "All selected checks completed successfully"
