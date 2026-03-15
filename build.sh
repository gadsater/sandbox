#!/usr/bin/env bash
set -euo pipefail

# Ensure wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "==> wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Ensure wasm32 target is available
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo "==> Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

echo "==> Building wasm package..."
wasm-pack build --target web --out-dir pkg

echo "==> Done. Serve the www/ directory to run the app."
echo "    Example: cd www && python3 -m http.server 8080"
