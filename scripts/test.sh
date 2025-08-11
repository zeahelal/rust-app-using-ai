#!/usr/bin/env bash
set -euo pipefail

RUSTFLAGS="-D warnings" cargo test --workspace --all-features
