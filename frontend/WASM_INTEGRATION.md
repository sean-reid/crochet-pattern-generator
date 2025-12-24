# WASM Integration Guide

This document explains how the frontend integrates with the Rust/WebAssembly backend for mesh processing and pattern generation.

## Overview

The frontend is designed to work with a compiled WebAssembly module that handles all computationally intensive operations:

- **Mesh Loading & Parsing**: Parse GLB/GLTF files and extract geometry
- **Mesh Analysis**: Calculate surface properties, curvature, topology
- **Surface Parameterization**: Create 2D UV mapping using LSCM/ABF++
- **Stitch Generation**: Generate stitch grid and classify types
- **Pattern Optimization**: Determine construction order and grouping
- **Instruction Generation**: Create human-readable instructions and SVG diagrams

## Expected WASM Module Location

The compiled WASM module should be placed at:
```
frontend/public/wasm/
├── crochet_pattern_wasm.js      # JS glue code
├── crochet_pattern_wasm_bg.wasm # Compiled WASM binary
└── crochet_pattern_wasm.d.ts    # TypeScript definitions
```

## WASM Interface

The frontend expects the following interface from the WASM module:

### Module Initialization

```typescript
import init from './wasm/crochet_pattern_wasm.js';

// Initialize the WASM module
await init();
```

### MeshProcessor Class

```typescript
export class MeshProcessor {
  /**
   * Create a new mesh processor from GLB/GLTF data
   * @param glbData - Binary data as Uint8Array
   */
  constructor(glbData: Uint8Array);

  /**
   * Get mesh information and statistics
   * @returns MeshInfo object with vertex/face counts, bounding box, etc.
   */
  get_mesh_info(): MeshInfo;

  /**
   * Generate crochet pattern from configuration
   * @param config - CrochetConfig object with gauge, yarn, construction settings
   * @returns CrochetPattern object with stitches, instructions, diagram
   */
  generate_pattern(config: CrochetConfig): CrochetPattern;

  /**
   * Simplify mesh to target face count
   * @param targetFaces - Desired number of faces after simplification
   */
  simplify_mesh(targetFaces: number): void;

  /**
   * Free WASM memory (must be called when done)
   */
  free(): void;
}
```

## Type Definitions

### MeshInfo

```typescript
interface MeshInfo {
  vertexCount: number;
  faceCount: number;
  boundingBox: {
    min: { x: number; y: number; z: number };
    max: { x: number; y: number; z: number };
  };
  surfaceArea: number;
  isManifold: boolean;
  hasUVs: boolean;
  hasNormals: boolean;
}
```

### CrochetConfig

```typescript
interface CrochetConfig {
  gauge: {
    stitchesPerInch: number;
    rowsPerInch: number;
  };
  yarn: {
    weight: 'lace' | 'fingering' | 'sport' | 'worsted' | 'bulky';
    hookSize: string; // e.g., "3.5mm" or "E/4"
  };
  construction: {
    type: 'flat' | 'amigurumi';
    stitchTypes: Array<'sc' | 'hdc' | 'dc'>;
  };
  optimization: {
    maxDistortion: number; // 0-1
    simplifyMesh: boolean;
    targetStitchCount?: number;
  };
}
```

### CrochetPattern

```typescript
interface CrochetPattern {
  metadata: {
    stitchCount: number;
    rowCount: number;
    estimatedTime: string;
    yarnEstimate: string;
    dimensions: { width: number; height: number; depth: number };
  };
  
  stitches: Array<{
    id: number;
    type: 'sc' | 'hdc' | 'dc' | 'inc' | 'dec';
    position3D: { x: number; y: number; z: number };
    position2D: { x: number; y: number };
    row: number;
    connections: number[]; // IDs of connected stitches
  }>;
  
  instructions: {
    rows: Array<{
      number: number;
      stitches: Array<{
        count: number;
        type: string;
        instruction: string;
      }>;
      totalStitches: number;
    }>;
  };
  
  diagram: {
    svg: string; // SVG markup
    stitchMap: Map<number, { x: number; y: number }>; // Stitch ID to diagram coords
  };
}
```

## Integration Points

### 1. File Upload (FileUploadZone)

**File**: `src/components/FileUploadZone/FileUploadZone.tsx`

```typescript
import { useModelLoader } from '../../hooks/useModelLoader';

const { loadModel } = useModelLoader();

// When file is selected
const result = await loadModel(file);
// This calls: processor = new MeshProcessor(glbData)
//             meshInfo = processor.get_mesh_info()
```

### 2. Pattern Generation (ConfigPanel)

**File**: `src/components/ConfigPanel/ConfigPanel.tsx`

```typescript
import { usePatternGeneration } from '../../hooks/usePatternGeneration';

const { generatePattern } = usePatternGeneration();

// When generate button is clicked
const result = await generatePattern(processor, config);
// This calls: pattern = processor.generate_pattern(config)
```

### 3. Memory Cleanup

**File**: `src/hooks/useWasmProcessor.ts`

```typescript
// When component unmounts or new file is loaded
cleanup();
// This calls: processor.free()
```

## Building the WASM Module

From the `wasm/` directory:

```bash
# Build for web
wasm-pack build --target web --out-dir ../frontend/public/wasm

# Or build for bundler (if using webpack/vite bundler mode)
wasm-pack build --target bundler --out-dir ../frontend/public/wasm
```

## Loading the WASM Module

The frontend uses a custom hook to load the WASM module:

**File**: `src/hooks/useWasmModule.ts`

```typescript
import { useWasmModule } from '../hooks/useWasmModule';

const App = () => {
  const { wasmModule, loading, loaded, error } = useWasmModule();

  if (loading) return <LoadingScreen />;
  if (error) return <ErrorScreen error={error} />;
  if (!loaded) return <WasmNotLoadedScreen />;

  return <MainApp />;
};
```

## Error Handling

The frontend defines custom error types for WASM operations:

```typescript
// Mesh loading errors
class MeshLoadError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'MeshLoadError';
  }
}

// Pattern generation errors
class PatternGenerationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'PatternGenerationError';
  }
}
```

## Progress Reporting

Pattern generation reports progress through stages:

```typescript
type PatternGenerationStage =
  | 'parameterization'      // 0-20%
  | 'grid-generation'       // 20-40%
  | 'stitch-classification' // 40-60%
  | 'optimization'          // 60-80%
  | 'instruction-generation'// 80-100%
  | 'complete';             // 100%
```

The WASM backend should update progress during long operations if possible.

## Memory Management

⚠️ **IMPORTANT**: Always call `processor.free()` when done to avoid memory leaks:

```typescript
useEffect(() => {
  return () => {
    if (processor) {
      processor.free();
    }
  };
}, [processor]);
```

## Testing Without WASM

During development, you can test the UI without the WASM module. The hooks will throw descriptive errors indicating WASM is not loaded:

```
WASM module not loaded. Please build the Rust backend and place the compiled WASM module in public/wasm/
```

## Performance Considerations

1. **Large Meshes**: Consider showing a progress indicator during `generate_pattern()`
2. **Memory**: Call `free()` promptly to release WASM memory
3. **Caching**: The WASM module is loaded once and reused across multiple operations
4. **Web Workers**: Future optimization could move WASM processing to a worker thread

## Debugging

Enable WASM debugging in browser DevTools:

```javascript
// In vite.config.ts
export default defineConfig({
  build: {
    sourcemap: true, // Enable source maps
  },
});
```

## Common Issues

### Issue: "Cannot find module './wasm/crochet_pattern_generator.js'"

**Solution**: Build and copy the WASM module to `public/wasm/`

### Issue: "Memory access out of bounds"

**Solution**: Ensure you're calling `free()` appropriately and not using the processor after freeing

### Issue: "Invalid GLB/GLTF file"

**Solution**: Validate the file format in the Rust parser and return a descriptive error

## Next Steps

1. Build the Rust/WASM backend following the spec
2. Run `wasm-pack build` to generate the module
3. Copy output to `frontend/public/wasm/`
4. Test the integration with real 3D models
5. Optimize performance for large meshes
6. Add error handling for edge cases

## Resources

- [wasm-bindgen Documentation](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-pack Guide](https://rustwasm.github.io/wasm-pack/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
