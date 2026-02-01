#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source .env if present
if [[ -f "$ROOT_DIR/.env" ]]; then
  source "$ROOT_DIR/.env"
fi

# Ensure data directory exists
mkdir -p "$ROOT_DIR/data"

# Set defaults
export PORT="${PORT:-8090}"
export DATABASE_URL="${DATABASE_URL:-sqlite:${ROOT_DIR}/data/schweinehund.sqlite}"
export NTFY_URL="${NTFY_URL:-http://127.0.0.1:8091/schweinehund}"

# Run the backend
cargo run --manifest-path "$ROOT_DIR/backend/Cargo.toml"
