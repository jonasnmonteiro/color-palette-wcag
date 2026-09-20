#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

if command -v wasm-pack &> /dev/null; then
  wasm-pack build --target web --out-dir ./pkg --release
else
  echo "wasm-pack not found, relying on isomorphic TypeScript engine"
fi

if command -v npm &> /dev/null; then
  npm run build || true
fi
