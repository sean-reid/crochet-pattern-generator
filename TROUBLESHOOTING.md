# Troubleshooting Guide

## WASM Build Issues

### Error: "bulk memory operations require bulk memory"

**Solution**: The `.cargo/config.toml` file in the `wasm/` directory enables bulk memory operations. Make sure it exists with the following content:

```toml
[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "link-arg=--no-entry",
  "-C", "target-feature=+bulk-memory",
  "-C", "target-feature=+mutable-globals",
  "-C", "target-feature=+nontrapping-fptoint",
]
```

Additionally, the `crochet-wasm/Cargo.toml` should have this section to tell wasm-opt about the features:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-O", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]
```

If the file is present and you still get this error, try:
```bash
cd wasm
cargo clean
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
```

### Error: "wasm*-unknown-unknown targets are not supported"

**Solution**: This is fixed by the `getrandom = { version = "0.2", features = ["js"] }` dependency in `crochet-core/Cargo.toml`. Make sure it's present.

### Error: "failed to load WASM module"

**Possible causes**:
1. WASM files not generated - rebuild with wasm-pack
2. CORS issues - make sure dev server has proper headers (Vite handles this)
3. Browser doesn't support WASM - check browser version

**Solution**:
```bash
cd wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
```

Check that files exist in `frontend/public/wasm/`:
- `crochet_wasm.js`
- `crochet_wasm_bg.wasm`
- `crochet_wasm_bg.wasm.d.ts`

## Frontend Build Issues

### Error: "Cannot find module '/wasm/crochet_wasm.js'"

**Solution**: Build the WASM module first before starting the frontend:
```bash
cd wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
cd ../frontend
npm run dev
```

### Error: "Failed to initialize WASM"

**Check**:
1. Browser console for specific error
2. Network tab - is the .wasm file loading?
3. Check file permissions on WASM files

### Error: npm install fails

**Solution**: Make sure you have Node.js 18+ installed:
```bash
node --version  # Should be 18.0.0 or higher
npm install
```

## Runtime Issues

### Pattern generation hangs

**Check**:
1. Open browser DevTools console for errors
2. Check if Web Worker is blocked (some corporate firewalls block workers)
3. Try with a simpler profile (fewer points)

### Drawing canvas not responding

**Solutions**:
1. Hard refresh: Cmd+Shift+R (Mac) or Ctrl+Shift+R (Windows/Linux)
2. Clear browser cache
3. Check console for JavaScript errors

### Pattern looks incorrect

**Verify**:
1. Profile curve has at least 2 points
2. Configuration values are reasonable:
   - Height: 5-30 cm typical
   - Diameters: 2-20 cm typical  
   - Gauge: 2.5-4.0 stitches/cm typical
   - Hook: 3.0-5.0 mm typical

## Development Issues

### Rust tests fail

```bash
cd wasm
cargo test --target x86_64-unknown-linux-gnu  # Or your native target
```

Don't run tests with WASM target - use native target for testing.

### Hot reload not working

**Solution**: 
1. Restart Vite dev server
2. Check if files are being saved properly
3. Ensure you're editing files in `frontend/src/`, not in `dist/`

### TypeScript errors

**Solution**:
```bash
cd frontend
npm run build  # This runs tsc to check types
```

Fix any type errors reported.

## Browser Compatibility

### "WebAssembly is not defined"

Your browser doesn't support WebAssembly. Upgrade to:
- Chrome 91+
- Firefox 90+
- Safari 15+
- Edge 91+

### Worker not supported

Your browser doesn't support Web Workers. Same version requirements as above.

## Performance Issues

### Pattern generation is slow (>10 seconds)

**Causes**:
1. Very complex profile (100+ points) - simplify curve
2. Debug build of WASM - use release build:
   ```bash
   cd wasm
   wasm-pack build crochet-wasm --target web --release --out-dir ../frontend/public/wasm
   ```

### UI is laggy while drawing

**Solutions**:
1. Disable grid if not needed
2. Reduce number of points
3. Check CPU usage - close other applications

## Getting Help

If you encounter an issue not listed here:

1. Check browser console for errors
2. Check terminal where dev server is running
3. Try building with verbose output:
   ```bash
   cd wasm
   cargo clean
   RUST_LOG=debug wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
   ```
4. Check that all dependencies are installed:
   ```bash
   rustc --version  # Should be 1.70+
   wasm-pack --version  # Should be 0.12+
   node --version  # Should be 18+
   ```

## Clean Build

If all else fails, do a clean build:

```bash
# Clean Rust build
cd wasm
cargo clean
rm -rf ../frontend/public/wasm

# Clean frontend build
cd ../frontend
rm -rf node_modules dist
npm install

# Rebuild everything
cd ../wasm
wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
cd ../frontend
npm run dev
```
