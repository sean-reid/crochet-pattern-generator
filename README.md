# Crochet Pattern Generator

A web application that generates optimized crochet patterns for amigurumi from user-drawn cross-sections. Draw a 2D profile, configure your yarn specifications, and get detailed stitch-by-stitch instructions.

## Features

- **Interactive Drawing Canvas**: Draw profile curves with cubic Bézier splines
- **Physical Yarn Specifications**: Configure gauge, hook size, and dimensions
- **Optimized Stitch Placement**: Even distribution of increases/decreases for visual consistency
- **3D Preview**: Real-time visualization of the resulting amigurumi
- **Pattern Export**: Generate PDF patterns with row-by-row instructions
- **High Performance**: Rust/WebAssembly computation engine

## Quick Start

### Prerequisites

- Node.js 18+ and npm
- Rust 1.70+ with wasm-pack
- Modern browser (Chrome 91+, Firefox 90+, Safari 15+, Edge 91+)

### Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd crochet-pattern-generator
   ```

2. **Build the WASM module**:
   ```bash
   cd wasm
   wasm-pack build crochet-wasm --target web --out-dir ../frontend/public/wasm
   cd ..
   ```

3. **Install frontend dependencies**:
   ```bash
   cd frontend
   npm install
   ```

4. **Start development server**:
   ```bash
   npm run dev
   ```

5. **Open your browser** to `http://localhost:5173`

## Project Structure

```
crochet-pattern-generator/
├── wasm/                      # Rust/WebAssembly backend
│   ├── Cargo.toml            # Workspace configuration
│   ├── crochet-core/         # Core algorithms (pure Rust)
│   ├── crochet-wasm/         # WASM bindings
│   └── crochet-types/        # Shared type definitions
├── frontend/                  # React/TypeScript frontend
│   ├── src/
│   │   ├── components/       # React components
│   │   ├── workers/          # Web Workers
│   │   ├── hooks/            # Custom React hooks
│   │   ├── types/            # TypeScript definitions
│   │   ├── utils/            # Helper functions
│   │   └── pages/            # Top-level page components
│   ├── public/               # Static assets
│   └── package.json
└── README.md
```

## Usage

### 1. Draw Your Profile

- Click on the canvas to add control points
- Drag points to adjust the curve
- The curve represents one half of your amigurumi (will be rotated 360°)
- Bottom of curve = starting magic circle
- Top of curve = ending magic circle

### 2. Configure Specifications

- **Total Height**: Final height in centimeters
- **Start/End Diameter**: Diameter at bottom and top
- **Yarn Gauge**: Stitches and rows per centimeter
- **Hook Size**: Recommended crochet hook size

### 3. Generate Pattern

- Click "Generate Pattern" to compute stitch instructions
- View 3D preview of the resulting shape
- Review row-by-row stitch counts and instructions

### 4. Export

- Export to PDF for printing
- Export to JSON for saving/sharing
- Copy text instructions to clipboard

## Technology Stack

- **Computation**: Rust compiled to WebAssembly (wasm-bindgen)
- **Frontend**: React 18 with TypeScript
- **Build Tools**: Vite (frontend), wasm-pack (Rust)
- **Styling**: Tailwind CSS
- **3D Graphics**: Three.js
- **Worker Communication**: Comlink for typed Web Worker RPC

## Development

### Building WASM for Production

```bash
cd wasm
wasm-pack build crochet-wasm --target web --release --out-dir ../frontend/public/wasm
```

### Building Frontend for Production

```bash
cd frontend
npm run build
```

The production build will be in `frontend/dist/`.

### Running Tests

**Rust tests**:
```bash
cd wasm
cargo test
```

**Frontend tests**:
```bash
cd frontend
npm test
```

## Performance

- Pattern generation: <3 seconds for typical amigurumi (30 rows, 60 stitches/row)
- UI responsiveness: <16ms frame time during drawing
- Initial load: <2 seconds on 3G connection
- WASM module size: ~400KB (compressed)

## Browser Compatibility

- Chrome 91+
- Firefox 90+
- Safari 15+
- Edge 91+

Requires WebAssembly and Web Worker support.

## Algorithm Overview

1. **Profile Sampling**: Uniform arc-length sampling of Bézier spline using adaptive Simpson integration
2. **Radius Calculation**: Extract radii with Gaussian smoothing
3. **Row Mapping**: Map continuous curve to discrete crochet rows based on gauge
4. **Stitch Count Calculation**: Compute stitches per row based on circumference and gauge
5. **Optimization**: Distribute increases/decreases evenly using simulated annealing with electrostatic repulsion model

## License

MIT License - See LICENSE file for details

## Contributing

Contributions welcome! Please read CONTRIBUTING.md for guidelines.

## Acknowledgments

- Mathematical foundations based on differential geometry and surface of revolution theory
- Optimization algorithms inspired by computational physics
- UI design follows modern minimalist principles with craft-inspired aesthetics

## Support

For issues, questions, or feature requests, please open an issue on GitHub.
