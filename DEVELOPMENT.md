# Development Guide

## Project Structure

```
crochet-pattern-generator/
├── wasm/                         # Rust/WASM backend
│   ├── Cargo.toml               # Workspace configuration
│   ├── crochet-types/           # Type definitions
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs           # Point2D, SplineSegment, CrochetPattern, etc.
│   ├── crochet-core/            # Core algorithms
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs           # Module exports
│   │       ├── sampling.rs      # Profile curve sampling
│   │       ├── radius.rs        # Radius calculation with smoothing
│   │       ├── row_mapping.rs   # Map samples to crochet rows
│   │       ├── stitch_count.rs  # Calculate stitches per row
│   │       ├── optimization.rs  # Stitch placement optimization
│   │       └── generator.rs     # Main pattern generation pipeline
│   └── crochet-wasm/            # WASM bindings
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs           # wasm-bindgen exports
└── frontend/                     # React frontend
    ├── package.json
    ├── vite.config.ts
    ├── tailwind.config.js
    ├── src/
    │   ├── main.tsx             # Entry point
    │   ├── App.tsx              # Main app component
    │   ├── index.css            # Global styles
    │   ├── types/
    │   │   └── index.ts         # TypeScript type definitions
    │   ├── components/
    │   │   ├── DrawingCanvas.tsx
    │   │   ├── ConfigurationPanel.tsx
    │   │   ├── PatternPreview.tsx
    │   │   └── ExportPanel.tsx
    │   └── workers/
    │       └── pattern-worker.ts # Web Worker for WASM
    └── public/
        ├── wasm/                 # Generated WASM files (git-ignored)
        └── icon.svg
```

## Development Workflow

### Initial Setup

1. **Install Rust and wasm-pack**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install wasm-pack
   ```

2. **Install Node.js dependencies**:
   ```bash
   cd frontend
   npm install
   cd ..
   ```

3. **Build WASM module**:
   ```bash
   cd wasm
   wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
   cd ..
   ```
   
   Note: The `.cargo/config.toml` enables bulk memory operations needed for modern WASM.

### Running Development Server

```bash
cd frontend
npm run dev
```

Open browser to `http://localhost:5173`

### Making Changes

#### Rust/WASM Changes

1. Edit files in `wasm/crochet-core/src/` or `wasm/crochet-wasm/src/`
2. Rebuild WASM:
   ```bash
   cd wasm
   wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
   cd ..
   ```
   
   The `.cargo/config.toml` handles the necessary WASM features automatically.
3. Refresh browser (Vite will hot-reload frontend)

#### Frontend Changes

1. Edit files in `frontend/src/`
2. Vite will automatically hot-reload changes

### Running Tests

**Rust tests**:
```bash
cd wasm
cargo test
```

**Individual module tests**:
```bash
cd wasm
cargo test -p crochet-core
cargo test -p crochet-wasm
```

### Production Build

```bash
./build.sh
```

Or manually:
```bash
# Build WASM with bulk memory support (configured in .cargo/config.toml)
cd wasm
wasm-pack build crochet-wasm --target web --release --out-dir ../frontend/public/wasm

# Build frontend
cd ../frontend
npm run build
```

Output: `frontend/dist/`

## Architecture Overview

### Data Flow

1. **User draws profile** → DrawingCanvas component
   - Stores points as `Point2D[]`
   - Generates cubic Bézier spline segments
   - Creates `ProfileCurve` object

2. **User configures dimensions** → ConfigurationPanel
   - Sets `AmigurumiConfig` (height, diameters, yarn gauge)
   - Validates inputs

3. **Pattern generation** → Web Worker
   - Passes `ProfileCurve` and `AmigurumiConfig` to WASM
   - WASM pipeline:
     - Sample curve uniformly (arc-length parameterization)
     - Calculate radius profile with Gaussian smoothing
     - Map to discrete crochet rows based on gauge
     - Calculate stitch counts per row
     - Optimize stitch placement (simulated annealing)
   - Returns `CrochetPattern`

4. **Display pattern** → PatternPreview
   - Shows row-by-row instructions
   - Displays metadata (time, yarn length)

5. **Export** → ExportPanel
   - Copy to clipboard
   - Download as text or JSON

### Key Algorithms

#### 1. Profile Sampling (`sampling.rs`)
- Uses adaptive Simpson integration for arc length
- Newton-Raphson for inverse arc-length lookup
- Ensures uniform spacing along curve

#### 2. Radius Calculation (`radius.rs`)
- Extracts x-coordinates from sampled points
- Applies Gaussian smoothing (σ = 0.5 × sample spacing)
- Clamps negative values to zero

#### 3. Row Mapping (`row_mapping.rs`)
- Calculates row height from gauge
- Binary search for nearest sample at each row height

#### 4. Stitch Count (`stitch_count.rs`)
- Circumference = 2πr
- Stitches = circumference × gauge
- Enforces max 16.7% change per row (for physical feasibility)

#### 5. Optimization (`optimization.rs`)
- Simulated annealing (1000 iterations)
- Energy function: electrostatic repulsion model
- Staggers special stitches between adjacent rows

### Performance Targets

- Pattern generation: <3s for 30 rows
- UI frame rate: 60 FPS during drawing
- Initial load: <2s on 3G
- WASM size: <500KB uncompressed

### Common Issues

#### WASM not loading
- Check browser console for CORS errors
- Ensure WASM files are in `frontend/public/wasm/`
- Rebuild WASM module

#### Pattern generation fails
- Validate profile has at least 2 points
- Check configuration values are positive
- Look for NaN or Infinity in calculations

#### Drawing canvas not responding
- Check React DevTools for state issues
- Verify event handlers are attached
- Clear browser cache

## Adding New Features

### New Stitch Type

1. Add to `StitchType` enum in `crochet-types/src/lib.rs`
2. Update `to_string()` method
3. Update pattern generation in `generator.rs`
4. Update frontend TypeScript types
5. Add UI for new stitch type

### New Optimization Strategy

1. Create new module in `crochet-core/src/`
2. Implement optimization function
3. Add to `optimization.rs` or call from `generator.rs`
4. Add tests
5. Expose via WASM if needed

### New Export Format

1. Add function to `ExportPanel.tsx`
2. Implement formatting logic
3. Add download button
4. Test with various patterns

## Code Style

### Rust
- Follow `rustfmt` defaults
- Use descriptive variable names
- Add doc comments for public APIs
- Write unit tests for algorithms

### TypeScript/React
- Use functional components with hooks
- Follow Prettier defaults
- Use TypeScript strict mode
- Avoid `any` types

### CSS/Tailwind
- Use Tailwind utility classes
- Follow design system in architecture doc
- Extract repeated patterns to CSS classes
- Maintain WCAG AA contrast ratios

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [React Docs](https://react.dev/)
- [Vite Docs](https://vitejs.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
