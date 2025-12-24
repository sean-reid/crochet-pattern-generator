# WASM Package Name Reference

## Important: Package Name

The WASM package is named: **`crochet_pattern_wasm`** (with hyphens)

## Build Command

From the `wasm/` directory:

```bash
wasm-pack build --target web --out-dir ../frontend/public/wasm
```

## Expected Output Files

After building, you should see:

```
frontend/public/wasm/
├── crochet_pattern_wasm.js          # JavaScript glue code
├── crochet_pattern_wasm_bg.wasm     # WebAssembly binary
├── crochet_pattern_wasm.d.ts        # TypeScript definitions
└── package.json                      # Package metadata
```

## How Frontend Imports It

The frontend will import like this:

```typescript
import init, { MeshProcessor } from './wasm/crochet_pattern_wasm.js';

// Initialize WASM
await init();

// Create processor
const processor = new MeshProcessor(glbData);

// Use it
const meshInfo = processor.get_mesh_info();
const pattern = processor.generate_pattern(config);
processor.free();
```

## Verification

After building, verify the files exist:

```bash
ls -la frontend/public/wasm/
```

You should see:
- `crochet_pattern_wasm.js` (JavaScript wrapper)
- `crochet_pattern_wasm_bg.wasm` (Binary WASM)
- `crochet_pattern_wasm.d.ts` (TypeScript types)

## Troubleshooting

If you get "Cannot find module" errors:
1. Check that the files are in `public/wasm/`
2. Verify the package name matches `crochet_pattern_wasm` (not `crochet_pattern_generator`)
3. Restart the dev server after adding WASM files

## Current Status

✅ Frontend ready and waiting for WASM module
✅ All import paths configured for `crochet_pattern_wasm`
✅ Error messages show helpful feedback when WASM not loaded
❌ WASM module needs to be built from Rust backend

Once the WASM module is built and placed in `public/wasm/`, the frontend will automatically detect and use it!
