#!/usr/bin/env bash
set -euo pipefail

echo "==> Building wasm package..."
wasm-pack build --target web --out-dir pkg

echo "==> Done. Serve the www/ directory to run the app."
echo "    Example: cd www && python3 -m http.server 8080"
