#!/bin/bash
set -e

echo "Building Crochet Pattern Generator for GitHub Pages..."

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Ensure a clean start for the docs folder
rm -rf "$PROJECT_ROOT/docs"

# Build WASM module
echo "Building WASM module..."
cd "$PROJECT_ROOT/wasm"
wasm-pack build crochet-wasm --target web --out-dir "$PROJECT_ROOT/frontend/public/wasm"
cd "$PROJECT_ROOT"

# Build frontend
echo "Building frontend..."
cd "$PROJECT_ROOT/frontend"
npm install
npm run build # This will now output to ../docs per vite.config.ts
cd "$PROJECT_ROOT"

echo "Build complete!"
echo "Production files are in the /docs folder. You can now push to GitHub and set Pages to host from /docs."
