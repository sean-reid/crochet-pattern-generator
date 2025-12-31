# Development Guide

## Project Structure

```
wasm/crochet-core/src/
  ├── generator.rs      # Main pattern generation
  ├── stitch_count.rs   # Radius → stitch conversion
  ├── optimization.rs   # Stitch placement optimization
  └── sampling.rs       # Arc-length curve sampling (not currently used)

frontend/src/
  ├── App.tsx                          # Main app with tab navigation
  ├── components/DrawingCanvas.tsx     # 2D drawing + 3D preview
  ├── components/ConfigurationPanel.tsx
  ├── components/PatternPreview.tsx
  └── components/ExportPanel.tsx
```

## Key Algorithms

### Pattern Generation (`generator.rs`)

```rust
1. Sample curve at evenly-spaced heights (not arc-length!)
   - Rows are horizontal circles at specific heights
   - Height spacing = 1 / gauge_rows_per_cm
   
2. Find radius at each height via binary search on curve

3. Convert radius to stitches
   - Row 1: 6 SC (magic ring, standard)
   - Row 2+: circumference × gauge
   
4. Apply physical constraints
   - Max 2× increase per row (all INC stitches)
   - Max 0.5× decrease per row (all INVDEC)
   
5. Generate stitch instructions (INC/DEC distribution)

6. Optimize placement (simulated annealing)
```

### Drawing (`DrawingCanvas.tsx`)

- **Spline:** Cubic B-splines (smooth, C2 continuous)
- **Storage:** Control points stored in App.tsx state
- **3D Preview:** Three.js surface of revolution
- **Controls:** Rotate (left-drag), Pan (right-drag), Zoom (scroll), Reset (double-click)

## Common Tasks

### Rebuild WASM After Rust Changes
```bash
cd wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
```

### Run Tests
```bash
cd wasm
cargo test
```

### Clean Build
```bash
cd wasm
cargo clean
rm -rf ../frontend/public/wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm

cd ../frontend
rm -rf node_modules dist
npm install
npm run dev
```

## Important Design Decisions

### Why Height-Based Sampling?
Crochet rows are worked at specific heights (horizontal circles). Arc-length sampling clusters samples at steep sections, causing incorrect row distribution.

### Why Physical Constraints?
Without constraints, pattern can request impossible changes (e.g., 12 stitches → 105 stitches in one row). Constraints based on crochet mechanics:
- INC: consumes 1, produces 2 → max 100% increase
- INVDEC: consumes 2, produces 1 → max 50% decrease

### Why B-Splines?
Guaranteed C2 continuity (smooth curvature). Catmull-Rom splines can create cusps.

### Why Magic Ring = 6 SC?
Standard amigurumi practice. Not calculated from circumference - it's a fixed starting technique.

## File Changes Workflow

1. Edit Rust files
2. Rebuild WASM (see above)
3. Browser auto-reloads via Vite HMR
4. Test in browser
5. Run `cargo test` if algorithm changed

## Debugging

**Pattern generation fails:**
- Check browser console for WASM error
- Common: curve discontinuity, negative radii, validation errors

**Drawing doesn't persist:**
- Control points should be in App.tsx state
- onChange should pass both profile AND points

**3D preview doesn't show:**
- Check Three.js is installed: `npm ls three`
- Check browser console for WebGL errors

## Dependencies

**Rust:**
- serde, serde_json (serialization)
- wasm-bindgen (JS interop)
- rand, rand_chacha (optimization RNG)

**Frontend:**
- React 18
- Three.js (3D preview)
- Vite (build tool)
- Tailwind CSS (styling)
