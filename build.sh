#!/bin/bash

set -e

echo "Building Crochet Pattern Generator..."

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Build WASM module
echo "Building WASM module..."
cd "$PROJECT_ROOT/wasm"
wasm-pack build crochet-wasm --target web --out-dir "$PROJECT_ROOT/frontend/public/wasm"
cd "$PROJECT_ROOT"

# Build frontend
echo "Building frontend..."
cd "$PROJECT_ROOT/frontend"
npm install
npm run build
cd "$PROJECT_ROOT"

echo "Build complete!"
echo "Production files are in frontend/dist/"
