# Crochet Pattern Generator

Generate amigurumi crochet patterns from drawn 2D profiles.

## Features

- Draw 2D profile curves with smooth B-splines
- Real-time 3D preview of rotated shape
- Automatic stitch pattern generation
- Configurable yarn gauge and dimensions
- Export patterns as text or JSON

## Quick Start

```bash
# 1. Build WASM
cd wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm

# 2. Run frontend
cd ../frontend
npm install
npm run dev
```

Open `http://localhost:5173`

## How It Works

1. **Draw**: Create a profile curve (one side of your amigurumi)
2. **Configure**: Set height and yarn gauge
3. **Generate**: Algorithm creates row-by-row stitch instructions
4. **Export**: Download or copy the pattern

## Core Algorithm

- Height-based sampling (rows at specific vertical positions)
- Radius → circumference → stitch count
- Physical constraints: max 2× increase, 0.5× decrease per row
- Stitch placement optimization for even distribution

## Project Structure

```
wasm/                    # Rust backend
  crochet-types/         # Type definitions
  crochet-core/          # Core algorithms
  crochet-wasm/          # WASM bindings
frontend/                # React frontend
  src/components/        # UI components
  src/workers/           # Web Worker for WASM
```

## Requirements

- Rust 1.70+
- wasm-pack
- Node.js 18+
- Modern browser with WebAssembly support

## Development

**Rust changes:**
```bash
cd wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
```

**Frontend changes:**
Vite hot-reloads automatically.

**Tests:**
```bash
cd wasm
cargo test
```

## Notes

- Patterns use standard magic ring (6 SC)
- Supports convex shapes only (one radius per height)
- B-spline curves for smooth profiles
- Patterns close with 6-15 stitches for hand-sewing
