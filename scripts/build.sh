#!/usr/bin/env bash
set -euo pipefail

# Build backend
cargo build -p backend

# Optionally build frontend (requires trunk)
if command -v trunk >/dev/null 2>&1; then
  trunk build --config frontend/Trunk.toml
else
  echo "trunk not found, skipping frontend build" >&2
fi
