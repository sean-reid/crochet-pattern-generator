# Frontend Integration Guide

## Overview

This WASM module compiles to `frontend/public/wasm/` and can be imported directly in your React components.

## Build Command

```bash
# From project root
cd wasm
wasm-pack build --target web --out-dir ../frontend/public/wasm

# Or use the provided script
./build-wasm.sh
```

## Import in React/TypeScript

### Method 1: Direct Import (Recommended)

```typescript
// src/hooks/useWasmProcessor.ts
import init, { 
  load_model, 
  generate_pattern,
  validate_model,
  get_mesh_info 
} from '../../public/wasm/crochet_pattern_wasm.js';

let wasmInitialized = false;

export const useWasmProcessor = () => {
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    if (!wasmInitialized) {
      init().then(() => {
        wasmInitialized = true;
        setIsReady(true);
      });
    } else {
      setIsReady(true);
    }
  }, []);

  return { isReady };
};
```

### Method 2: Dynamic Import (Code Splitting)

```typescript
const loadWasm = async () => {
  const wasm = await import('../../public/wasm/crochet_pattern_wasm.js');
  await wasm.default(); // Initialize
  return wasm;
};
```

## API Usage Examples

### 1. Load and Validate Model

```typescript
const handleFileUpload = async (file: File) => {
  const arrayBuffer = await file.arrayBuffer();
  const data = new Uint8Array(arrayBuffer);
  
  try {
    // Validate the model
    const validation = await validate_model(data);
    
    if (!validation.valid) {
      console.error('Invalid model:', validation.error);
      return;
    }
    
    // Get basic info
    const info = await get_mesh_info(data);
    console.log(`Model: ${info.vertices} vertices, ${info.faces} faces`);
    
    return { data, info };
  } catch (error) {
    console.error('Failed to load model:', error);
  }
};
```

### 2. Generate Pattern

```typescript
const generateCrochetPattern = async (modelData: Uint8Array) => {
  const config = {
    stitches_per_inch: 5.0,
    rows_per_inch: 5.5,
    yarn_weight: "worsted",
    hook_size_mm: 5.0,
    target_width_inches: 6.0,
    target_height_inches: null,
    construction_type: "flat",
    max_distortion: 0.3,
    simplify_mesh: true,
    target_stitch_count: 500
  };
  
  try {
    const result = await generate_pattern(modelData, config);
    
    if (result.success && result.pattern) {
      const pattern = result.pattern;
      
      console.log('Pattern metadata:', {
        stitchCount: pattern.metadata.stitch_count,
        rowCount: pattern.metadata.row_count,
        estimatedTime: pattern.metadata.estimated_time,
        yarnEstimate: pattern.metadata.yarn_estimate
      });
      
      return pattern;
    } else {
      console.error('Generation failed:', result.error);
    }
  } catch (error) {
    console.error('Exception during generation:', error);
  }
};
```

### 3. Export Pattern

```typescript
const exportPattern = async (pattern: any, format: 'json' | 'svg' | 'text') => {
  try {
    const exported = await export_pattern(pattern, format);
    
    // Create download
    const blob = new Blob([exported], { 
      type: format === 'json' ? 'application/json' : 
            format === 'svg' ? 'image/svg+xml' : 
            'text/plain' 
    });
    
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `pattern.${format}`;
    a.click();
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('Export failed:', error);
  }
};
```

## Complete Hook Example

```typescript
// src/hooks/usePatternGenerator.ts
import { useState, useEffect } from 'react';
import init, { generate_pattern } from '../../public/wasm/crochet_pattern_wasm.js';

export const usePatternGenerator = () => {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    init()
      .then(() => setIsInitialized(true))
      .catch(err => setError(err.message));
  }, []);

  const generate = async (modelData: Uint8Array, config: any) => {
    if (!isInitialized) {
      setError('WASM not initialized');
      return null;
    }

    setIsGenerating(true);
    setError(null);

    try {
      const result = await generate_pattern(modelData, config);
      
      if (result.success) {
        return result.pattern;
      } else {
        setError(result.error || 'Unknown error');
        return null;
      }
    } catch (err: any) {
      setError(err.message);
      return null;
    } finally {
      setIsGenerating(false);
    }
  };

  return {
    isInitialized,
    isGenerating,
    error,
    generate
  };
};
```

## Type Definitions

Create `src/types/wasm.d.ts`:

```typescript
// Augment the generated types
declare module '../../public/wasm/crochet_pattern_wasm.js' {
  export default function init(): Promise<void>;
  
  export function load_model(data: Uint8Array): Promise<any>;
  export function generate_pattern(data: Uint8Array, config: any): Promise<any>;
  export function validate_model(data: Uint8Array): Promise<any>;
  export function get_mesh_info(data: Uint8Array): Promise<any>;
  export function export_pattern(pattern: any, format: string): Promise<string>;
}
```

## Configuration Schema

```typescript
interface CrochetConfig {
  stitches_per_inch: number;      // Typically 4-8
  rows_per_inch: number;           // Typically 4-8
  yarn_weight: string;             // "lace" | "fingering" | "sport" | "dk" | "worsted" | "bulky"
  hook_size_mm: number;            // 2.0 - 10.0
  target_width_inches: number | null;
  target_height_inches: number | null;
  construction_type: string;       // "flat" | "amigurumi" | "top_down" | "bottom_up"
  max_distortion: number;          // 0.0 - 1.0
  simplify_mesh: boolean;
  target_stitch_count: number | null;
}
```

## Pattern Output Schema

```typescript
interface CrochetPattern {
  metadata: {
    stitch_count: number;
    row_count: number;
    estimated_time: string;
    yarn_estimate: string;
    dimensions: {
      width: number;
      height: number;
      depth: number;
    };
  };
  
  stitches: Array<{
    id: number;
    type: 'sc' | 'hdc' | 'dc' | 'inc' | 'dec';
    position_3d: [number, number, number];
    position_2d: [number, number];
    row: number;
    connections: number[];
  }>;
  
  instructions: {
    rows: Array<{
      number: number;
      stitches: Array<{
        count: number;
        type: string;
        instruction: string;
      }>;
      total_stitches: number;
    }>;
  };
  
  diagram?: string; // SVG markup
}
```

## Error Handling

```typescript
try {
  const pattern = await generate_pattern(data, config);
} catch (error) {
  if (error instanceof Error) {
    // Check for specific error types
    if (error.message.includes('Invalid GLTF')) {
      // Handle invalid file format
    } else if (error.message.includes('mesh')) {
      // Handle mesh processing error
    }
  }
}
```

## Performance Tips

1. **Initialize once**: Call `init()` only once at app startup
2. **Show loading states**: Pattern generation can take 100-500ms
3. **Worker threads**: Consider using Web Workers for heavy processing
4. **Caching**: Cache generated patterns to avoid recomputation
5. **Simplification**: Use `simplify_mesh: true` for faster processing

## Vite Configuration

Ensure your `vite.config.ts` serves WASM correctly:

```typescript
export default defineConfig({
  // ... other config
  server: {
    fs: {
      allow: ['..'] // Allow serving from parent directory
    }
  },
  build: {
    target: 'esnext' // Support for WebAssembly
  }
});
```

## Testing

```typescript
import { describe, it, expect, beforeAll } from 'vitest';
import init, { validate_model } from '../../public/wasm/crochet_pattern_wasm.js';

describe('WASM Integration', () => {
  beforeAll(async () => {
    await init();
  });

  it('validates a simple model', async () => {
    // Load test GLB file
    const response = await fetch('/test-model.glb');
    const data = new Uint8Array(await response.arrayBuffer());
    
    const result = await validate_model(data);
    expect(result.valid).toBe(true);
  });
});
```
