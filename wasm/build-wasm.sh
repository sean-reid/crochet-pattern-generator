#!/bin/bash

# Build script for WASM module
# This builds the Rust WASM and places it in frontend/public/wasm/

set -e  # Exit on error

echo "🎯 Building WASM module..."

# Navigate to wasm directory
cd wasm

# Build with wasm-pack targeting web
echo "📦 Running wasm-pack build..."
wasm-pack build --target web --out-dir ../frontend/public/wasm

echo "✅ WASM module built successfully!"
echo "📁 Output location: frontend/public/wasm/"

# List generated files
echo ""
echo "Generated files:"
ls -lh ../frontend/public/wasm/

echo ""
echo "🚀 You can now use the WASM module in your frontend:"
echo "   import init, { generate_pattern } from './wasm/crochet_pattern_wasm.js';"
