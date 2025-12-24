# Crochet Pattern Generator - WASM Module

WebAssembly module for converting 3D meshes (GLTF/GLB format) into crochet patterns.

## Overview

This Rust library compiles to WebAssembly and provides the computational core for analyzing 3D meshes and generating stitch-by-stitch crochet instructions. It implements sophisticated algorithms for:

- **Mesh Processing**: Loading, validation, simplification, and analysis of 3D models
- **Surface Parameterization**: LSCM (Least Squares Conformal Maps) for flattening 3D surfaces to 2D
- **Stitch Generation**: Creating a grid of stitches with appropriate types based on curvature
- **Pattern Optimization**: Organizing stitches into rows and generating human-readable instructions
- **Instruction Generation**: Producing text instructions and SVG diagrams

## Architecture

```
wasm/
├── src/
│   ├── lib.rs                  # Entry point and configuration
│   ├── bindings.rs             # WASM-JavaScript interface
│   ├── utils.rs                # Utility functions
│   │
│   ├── loader/                 # GLTF/GLB parsing
│   │   ├── gltf_parser.rs      # Parse and extract mesh data
│   │   └── validation.rs       # Mesh validation and checks
│   │
│   ├── mesh/                   # Mesh processing
│   │   ├── types.rs            # Core data structures
│   │   ├── processing.rs       # Normalization and cleanup
│   │   ├── analysis.rs         # Curvature computation
│   │   └── simplification.rs   # Edge collapse simplification
│   │
│   ├── parameterization/       # UV mapping
│   │   ├── lscm.rs             # Conformal parameterization
│   │   ├── abf.rs              # Angle-based flattening
│   │   ├── seam_placement.rs   # Seam cutting
│   │   └── distortion.rs       # Distortion analysis
│   │
│   ├── stitch/                 # Stitch generation
│   │   ├── grid_generator.rs   # Create stitch grid
│   │   ├── type_classifier.rs  # Classify stitch types
│   │   ├── connectivity.rs     # Build stitch graph
│   │   └── placement_optimizer.rs
│   │
│   ├── pattern/                # Pattern optimization
│   │   ├── optimizer.rs        # Main optimization
│   │   ├── row_grouping.rs     # Group similar rows
│   │   ├── amigurumi.rs        # In-the-round construction
│   │   └── types.rs            # Pattern data structures
│   │
│   ├── instruction/            # Instruction generation
│   │   ├── generator.rs        # Main generator
│   │   ├── diagram.rs          # SVG diagram creation
│   │   ├── abbreviations.rs    # Crochet abbreviations
│   │   └── formatter.rs        # Text formatting
│   │
│   └── algorithms/             # Supporting algorithms
│       ├── geodesic.rs         # Shortest paths on mesh
│       ├── curvature.rs        # Curvature computation
│       └── voronoi.rs          # Voronoi diagrams
│
├── tests/                      # Unit and integration tests
└── benches/                    # Performance benchmarks
```

## Building

### Prerequisites

- Rust (latest stable)
- wasm-pack: `cargo install wasm-pack`

### Build for Web

```bash
wasm-pack build --target web --out-dir ../frontend/src/wasm
```

### Build for Node.js

```bash
wasm-pack build --target nodejs
```

### Development Build

```bash
wasm-pack build --dev
```

## API

### Main Functions

#### `load_model(data: Uint8Array): Promise<Object>`

Load and validate a GLTF/GLB file.

**Returns**: Model metadata including vertex count, face count, and bounding box.

#### `generate_pattern(modelData: Uint8Array, config: CrochetConfig): Promise<CrochetPattern>`

Generate a complete crochet pattern from a 3D model.

**Parameters**:
- `modelData`: GLTF/GLB file bytes
- `config`: Crochet configuration object

**Returns**: Complete pattern with stitches, instructions, and diagram.

#### `validate_model(data: Uint8Array): Promise<ValidationResult>`

Validate a model and return warnings.

#### `export_pattern(pattern: CrochetPattern, format: string): Promise<string>`

Export pattern in various formats: "json", "svg", "text".

### Configuration

```typescript
interface CrochetConfig {
  stitchesPerInch: number;      // Gauge: stitches per inch
  rowsPerInch: number;           // Gauge: rows per inch
  yarnWeight: string;            // "lace" | "fingering" | "sport" | "dk" | "worsted" | "bulky"
  hookSizeMm: number;            // Hook size in millimeters
  targetWidthInches?: number;    // Optional target width
  targetHeightInches?: number;   // Optional target height
  constructionType: string;      // "flat" | "amigurumi" | "top_down" | "bottom_up"
  maxDistortion: number;         // 0.0 - 1.0
  simplifyMesh: boolean;         // Enable mesh simplification
  targetStitchCount?: number;    // Optional target stitch count
}
```

### Pattern Output

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
    position3D: [number, number, number];
    position2D: [number, number];
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
      totalStitches: number;
    }>;
  };
  diagram?: string;  // SVG markup
}
```

## Algorithms

### Surface Parameterization (LSCM)

Least Squares Conformal Maps minimize angle distortion when flattening the 3D surface to 2D. This preserves the shape locally, which is important for creating stitches that follow the surface naturally.

**Implementation**: Solves a sparse linear system using conjugate gradient.

### Curvature Analysis

Mean curvature at each vertex determines stitch type:
- High positive curvature → increases (more stitches)
- High negative curvature → decreases (fewer stitches)
- Low curvature → standard stitches

**Method**: Angle deficit method using the half-edge data structure.

### Mesh Simplification

Edge collapse algorithm reduces face count while preserving shape.

**Cost function**: Edge length (shorter edges collapsed first).

## Performance

- Typical processing time: 100-500ms for meshes with 1,000-10,000 faces
- Memory usage scales linearly with face count
- Supports progressive loading for large meshes

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Development

### Code Organization

- **lib.rs**: Public API and configuration
- **bindings.rs**: WASM-JavaScript bridge
- **modules**: Organized by functionality (loader, mesh, parameterization, etc.)

### Adding Features

1. Implement core algorithm in appropriate module
2. Add tests in `tests/`
3. Expose via `bindings.rs` if needed from JavaScript
4. Update this README

### Debugging

Use `console_error_panic_hook` for better error messages:

```rust
use crate::utils::log;

log("Debug message");
```

## Future Enhancements

- [ ] ABF++ parameterization (alternative to LSCM)
- [ ] Better seam placement algorithms
- [ ] Multi-color pattern support
- [ ] Custom stitch libraries
- [ ] Texture pattern integration
- [ ] Parallel processing with rayon
- [ ] Progressive mesh streaming
- [ ] Advanced construction methods (modular, top-down)

## License

See LICENSE file in repository root.

## Contributing

See CONTRIBUTING.md in repository root.
