import type { CrochetConfig } from './config';
import type { MeshInfo } from './mesh';
import type { CrochetPattern } from './pattern';

// WASM module exports standalone functions, not a class
export interface WasmModule {
  // Initialize the WASM module
  default: () => Promise<void>;
  
  // Load and validate a GLTF/GLB file
  load_model: (data: Uint8Array) => Promise<MeshInfo>;
  
  // Generate a complete crochet pattern
  generate_pattern: (modelData: Uint8Array, config: CrochetConfig) => Promise<CrochetPattern>;
  
  // Validate a model and return warnings
  validate_model: (data: Uint8Array) => Promise<ValidationResult>;
  
  // Export pattern in various formats
  export_pattern: (pattern: CrochetPattern, format: string) => Promise<string>;
  
  // Get mesh statistics
  get_mesh_info: (data: Uint8Array) => Promise<MeshInfo>;
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

// WASM loader state
export interface WasmLoadState {
  loading: boolean;
  loaded: boolean;
  error?: string;
}

// Error types from WASM
export class WasmError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'WasmError';
  }
}

export class MeshLoadError extends WasmError {
  constructor(message: string) {
    super(message);
    this.name = 'MeshLoadError';
  }
}

export class PatternGenerationError extends WasmError {
  constructor(message: string) {
    super(message);
    this.name = 'PatternGenerationError';
  }
}
