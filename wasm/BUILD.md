# Building the WASM Module

## Prerequisites

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack**
   ```bash
   cargo install wasm-pack
   ```

3. **wasm32-unknown-unknown target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Quick Build

From the project root:

```bash
cd wasm
wasm-pack build --target web --out-dir ../frontend/public/wasm
```

Or use the provided script:

```bash
./build-wasm.sh
```

## Build Output

The build will generate these files in `frontend/public/wasm/`:

- `crochet_pattern_wasm.js` - JavaScript bindings
- `crochet_pattern_wasm_bg.wasm` - Compiled WebAssembly module
- `crochet_pattern_wasm.d.ts` - TypeScript definitions
- `package.json` - Package metadata

## Integration in Frontend

### 1. Import the WASM module

```typescript
// In your React component or hook
import init, { 
  load_model, 
  generate_pattern,
  validate_model,
  get_mesh_info,
  export_pattern 
} from '../public/wasm/crochet_pattern_wasm.js';
```

### 2. Initialize the module

```typescript
// Initialize once when your app starts
useEffect(() => {
  const initWasm = async () => {
    try {
      await init();
      console.log('WASM module initialized');
    } catch (error) {
      console.error('Failed to initialize WASM:', error);
    }
  };
  
  initWasm();
}, []);
```

### 3. Use the API

```typescript
// Load and validate a model
const handleFileUpload = async (file: File) => {
  const arrayBuffer = await file.arrayBuffer();
  const data = new Uint8Array(arrayBuffer);
  
  // Validate first
  const validation = await validate_model(data);
  console.log('Validation:', validation);
  
  // Get mesh info
  const info = await get_mesh_info(data);
  console.log('Mesh info:', info);
};

// Generate pattern
const handleGeneratePattern = async (modelData: Uint8Array) => {
  const config = {
    stitches_per_inch: 5.0,
    rows_per_inch: 5.5,
    yarn_weight: "worsted",
    hook_size_mm: 5.0,
    target_width_inches: null,
    target_height_inches: null,
    construction_type: "flat",
    max_distortion: 0.3,
    simplify_mesh: true,
    target_stitch_count: null
  };
  
  const result = await generate_pattern(modelData, config);
  
  if (result.success) {
    console.log('Pattern generated:', result.pattern);
    // Use the pattern data
  } else {
    console.error('Pattern generation failed:', result.error);
  }
};

// Export pattern
const handleExport = async (pattern: any, format: string) => {
  const exported = await export_pattern(pattern, format);
  // 'format' can be: "json", "svg", or "text"
  return exported;
};
```

## Development Build

For faster builds during development:

```bash
wasm-pack build --dev --target web --out-dir ../frontend/public/wasm
```

## Production Build

For optimized production builds:

```bash
wasm-pack build --release --target web --out-dir ../frontend/public/wasm
```

This enables:
- Maximum optimization (opt-level = 3)
- Link-time optimization (LTO)
- Single codegen unit for better optimization

## Troubleshooting

### Build fails with "cargo metadata" error
- Ensure Cargo.toml is valid
- Run `cargo check` in the wasm directory

### "Could not find wasm-bindgen" error
- Reinstall wasm-pack: `cargo install wasm-pack --force`

### Module not found in frontend
- Check the import path matches the output directory
- Ensure the build completed successfully
- Verify files exist in `frontend/public/wasm/`

### WASM module fails to initialize
- Check browser console for errors
- Ensure you're serving the files over HTTP/HTTPS (not file://)
- Verify MIME types are correct for .wasm files

## File Structure After Build

```
frontend/
└── public/
    └── wasm/
        ├── crochet_pattern_wasm.js      # JS bindings
        ├── crochet_pattern_wasm_bg.wasm # WASM binary
        ├── crochet_pattern_wasm.d.ts    # TypeScript types
        ├── package.json                  # Package info
        └── .gitignore                    # Auto-generated
```

## TypeScript Integration

The generated `.d.ts` file provides full TypeScript support:

```typescript
// These types are automatically available
import type { 
  load_model,
  generate_pattern,
  validate_model
} from '../public/wasm/crochet_pattern_wasm';
```

## Performance Notes

- WASM module typically loads in < 100ms
- First-time compilation cached by browser
- Pattern generation: 100-500ms for typical meshes (1k-10k faces)
- Consider showing loading states for operations > 200ms

## CI/CD Integration

Add to your build pipeline:

```yaml
# .github/workflows/build.yml
- name: Build WASM
  run: |
    cd wasm
    wasm-pack build --target web --out-dir ../frontend/public/wasm
```

## Updating the Module

After making changes to Rust code:

1. Rebuild: `wasm-pack build --target web --out-dir ../frontend/public/wasm`
2. Restart dev server (Vite will pick up changes)
3. Hard refresh browser (Cmd+Shift+R / Ctrl+Shift+R)
