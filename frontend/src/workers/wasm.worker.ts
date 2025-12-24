/**
 * Web Worker for WASM mesh processing and pattern generation
 * Runs in a separate thread to keep UI responsive during heavy computations
 * 
 * The WASM module exports standalone functions, not a class:
 * - load_model(data)
 * - generate_pattern(modelData, config)
 * - validate_model(data)
 * - get_mesh_info(data)
 */

import type { CrochetConfig } from '../types/config';
import { toWasmConfig } from '../types/config';
import type { MeshInfo } from '../types/mesh';
import type { CrochetPattern } from '../types/pattern';

// Message types for worker communication
export type WorkerMessageType =
  | 'INIT_WASM'
  | 'LOAD_MESH'
  | 'GENERATE_PATTERN'
  | 'GET_MESH_INFO'
  | 'VALIDATE_MODEL';

export type WorkerProgressType =
  | 'parameterization'
  | 'grid-generation'
  | 'stitch-classification'
  | 'optimization'
  | 'instruction-generation';

// Messages from main thread to worker
export interface WorkerRequest {
  id: string;
  type: WorkerMessageType;
  payload?: {
    glbData?: Uint8Array;
    config?: CrochetConfig;
  };
}

// Messages from worker to main thread
export interface WorkerResponse {
  id: string;
  type: 'SUCCESS' | 'ERROR' | 'PROGRESS';
  payload?: {
    meshInfo?: MeshInfo;
    pattern?: CrochetPattern;
    validation?: any;
    progress?: {
      stage: WorkerProgressType;
      progress: number;
      message: string;
    };
  };
  error?: string;
}

// WASM module state
let wasmModule: any = null;
let wasmInitialized = false;
let currentModelData: Uint8Array | null = null;

/**
 * Initialize the WASM module
 */
async function initWasm(): Promise<void> {
  if (wasmInitialized) return;

  try {
    // Dynamic import of WASM module
    const wasm = await import('../../public/wasm/crochet_pattern_wasm.js');
    
    // Initialize WASM
    await wasm.default();
    
    // Store the module with all exported functions
    wasmModule = wasm;
    
    wasmInitialized = true;
    
    console.log('✅ WASM module loaded successfully in Web Worker');
  } catch (error) {
    throw new Error(`Failed to initialize WASM: ${error instanceof Error ? error.message : 'Unknown error'}`);
  }
}

/**
 * Load a mesh from GLB data and get mesh info
 */
async function loadMesh(glbData: Uint8Array): Promise<MeshInfo> {
  if (!wasmInitialized || !wasmModule) {
    throw new Error('WASM not initialized');
  }

  // Store the model data for later pattern generation
  currentModelData = glbData;
  
  // Call WASM function to load and analyze model
  const meshInfo = await wasmModule.load_model(glbData);
  
  return meshInfo;
}

/**
 * Get mesh info without storing model data
 */
async function getMeshInfo(glbData: Uint8Array): Promise<MeshInfo> {
  if (!wasmInitialized || !wasmModule) {
    throw new Error('WASM not initialized');
  }

  const meshInfo = await wasmModule.get_mesh_info(glbData);
  return meshInfo;
}

/**
 * Generate a crochet pattern
 */
async function generatePattern(config: CrochetConfig): Promise<CrochetPattern> {
  if (!wasmInitialized || !wasmModule) {
    throw new Error('WASM not initialized');
  }

  if (!currentModelData) {
    throw new Error('No model loaded. Call LOAD_MESH first.');
  }

  // Convert frontend config to WASM config format (camelCase -> snake_case)
  const wasmConfig = toWasmConfig(config);

  // Call WASM function to generate pattern
  // This function handles the entire pipeline:
  // - Surface parameterization (LSCM)
  // - Stitch grid generation
  // - Stitch type classification
  // - Pattern optimization
  // - Instruction generation
  const pattern = await wasmModule.generate_pattern(currentModelData, wasmConfig);
  
  return pattern;
}

/**
 * Validate a model
 */
async function validateModel(glbData: Uint8Array): Promise<any> {
  if (!wasmInitialized || !wasmModule) {
    throw new Error('WASM not initialized');
  }

  const validation = await wasmModule.validate_model(glbData);
  return validation;
}

/**
 * Handle messages from the main thread
 */
self.onmessage = async (event: MessageEvent<WorkerRequest>) => {
  const { id, type, payload } = event.data;

  try {
    switch (type) {
      case 'INIT_WASM': {
        await initWasm();
        const response: WorkerResponse = {
          id,
          type: 'SUCCESS',
        };
        self.postMessage(response);
        break;
      }

      case 'LOAD_MESH': {
        if (!payload?.glbData) {
          throw new Error('No GLB data provided');
        }
        const meshInfo = await loadMesh(payload.glbData);
        const response: WorkerResponse = {
          id,
          type: 'SUCCESS',
          payload: { meshInfo },
        };
        self.postMessage(response);
        break;
      }

      case 'GET_MESH_INFO': {
        if (!payload?.glbData) {
          throw new Error('No GLB data provided');
        }
        const meshInfo = await getMeshInfo(payload.glbData);
        const response: WorkerResponse = {
          id,
          type: 'SUCCESS',
          payload: { meshInfo },
        };
        self.postMessage(response);
        break;
      }

      case 'GENERATE_PATTERN': {
        if (!payload?.config) {
          throw new Error('No config provided');
        }
        const pattern = await generatePattern(payload.config);
        const response: WorkerResponse = {
          id,
          type: 'SUCCESS',
          payload: { pattern },
        };
        self.postMessage(response);
        break;
      }

      case 'VALIDATE_MODEL': {
        if (!payload?.glbData) {
          throw new Error('No GLB data provided');
        }
        const validation = await validateModel(payload.glbData);
        const response: WorkerResponse = {
          id,
          type: 'SUCCESS',
          payload: { validation },
        };
        self.postMessage(response);
        break;
      }

      default: {
        throw new Error(`Unknown message type: ${type}`);
      }
    }
  } catch (error) {
    const response: WorkerResponse = {
      id,
      type: 'ERROR',
      error: error instanceof Error ? error.message : 'Unknown error',
    };
    self.postMessage(response);
  }
};

// Export types for main thread
export {};
