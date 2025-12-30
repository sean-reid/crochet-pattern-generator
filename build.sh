#!/bin/bash

set -e

echo "Building Crochet Pattern Generator..."

# Build WASM module
echo "Building WASM module..."
cd wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
cd ..

# Build frontend
echo "Building frontend..."
cd frontend
npm install
npm run build
cd ..

echo "Build complete!"
echo "Production files are in frontend/dist/"
