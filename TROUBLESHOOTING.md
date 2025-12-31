# Troubleshooting

## "Failed to generate pattern"

**Check browser console (F12) for specific error.**

Common causes:

**"Curve must have positive height"**
- Curve endpoints at same y-coordinate
- Draw curve from bottom to top

**"Row X: pattern produces Y but expects Z"**
- Internal bug in stitch distribution
- Report with your curve shape

**"Failed to sample curve"**
- Invalid B-spline segments
- Try redrawing with fewer points

## WASM not loading

```bash
# Rebuild WASM
cd wasm
cargo clean
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm

# Check files exist
ls ../frontend/public/wasm/
# Should see: crochet_wasm.js, crochet_wasm_bg.wasm
```

## Drawing resets when switching tabs

Control points should persist in App.tsx state.

**Check:**
- DrawingCanvas receives `initialPoints` prop
- DrawingCanvas onChange calls: `onChange(profile, points)`
- App.tsx stores `controlPoints` state

## Pattern doesn't close (ends at 20+ stitches)

Physical constraints prevent closing if curve doesn't taper enough.

**Solutions:**
- Draw sharper taper at top
- Increase height (more rows to decrease)
- Accept 10-15 stitch closure (hand-sew it)

## 3D preview doesn't show

```bash
# Install Three.js
cd frontend
npm install three
```

Check browser console for WebGL errors.

## Rows 2-5 all have 6 stitches

Curve radius is too small near bottom.

**Solution:** Draw curve starting with visible radius (not at x=0).

## Clean rebuild

```bash
cd wasm
cargo clean
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm

cd ../frontend  
rm -rf node_modules dist .vite
npm install
npm run dev
```

Hard refresh browser: Ctrl+Shift+R
