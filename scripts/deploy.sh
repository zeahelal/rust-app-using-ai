#!/usr/bin/env bash
set -euo pipefail

# Build release binary
cargo build -p backend --release

# Example PM2 deploy using ecosystem.config.js
if command -v pm2 >/dev/null 2>&1; then
  pm2 startOrReload ecosystem.config.js --only backend
else
  echo "pm2 not installed; skipping process manager deploy" >&2
fi
