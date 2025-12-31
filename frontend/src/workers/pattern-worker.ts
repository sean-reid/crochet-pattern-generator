/// <reference path="../types/wasm.d.ts" />
import type { ProfileCurve, AmigurumiConfig, CrochetPattern } from '../types';

// WASM function references
let generate_pattern_from_json: any = null;
let validate_profile: any = null;
let validate_config: any = null;
let init_panic_hook: any = null;
let isInitialized = false;

async function initWasm(): Promise<void> {
  if (isInitialized) return;
  try {
    const wasmModule = await import(/* @vite-ignore */ '../wasm/crochet_wasm.js');
    await wasmModule.default();
    
    generate_pattern_from_json = wasmModule.generate_pattern_from_json;
    validate_profile = wasmModule.validate_profile;
    validate_config = wasmModule.validate_config;
    init_panic_hook = wasmModule.init_panic_hook;
    
    if (init_panic_hook) init_panic_hook();
    isInitialized = true;
  } catch (error) {
    console.error('WASM initialization failed:', error);
    throw new Error(`Failed to initialize WASM: ${(error as Error).message}`);
  }
}

// Define the shape of incoming messages for internal type safety
interface WorkerRequest {
  id: string;
  type: 'GENERATE_PATTERN' | 'VALIDATE_PROFILE' | 'VALIDATE_CONFIG';
  payload: any;
}

self.onmessage = async (e: MessageEvent<WorkerRequest>) => {
  const { id, type, payload } = e.data;

  try {
    await initWasm();

    switch (type) {
      case 'GENERATE_PATTERN': {
        // Explicitly casting payload ensures the types are "used" by TS
        const { profile, config } = payload as { 
          profile: ProfileCurve; 
          config: AmigurumiConfig 
        };
        
        // WASM function returns Result<String, String>
        try {
          const resultJson = generate_pattern_from_json(
            JSON.stringify(profile),
            JSON.stringify(config)
          );
          const pattern: CrochetPattern = JSON.parse(resultJson);
          
          self.postMessage({ id, type: 'SUCCESS', payload: pattern });
        } catch (wasmError) {
          // WASM threw an error
          throw new Error(`Pattern generation failed: ${wasmError}`);
        }
        break;
      }

      case 'VALIDATE_PROFILE': {
        const profile = payload as ProfileCurve;
        const result = validate_profile(JSON.stringify(profile));
        self.postMessage({ id, type: 'SUCCESS', payload: result });
        break;
      }

      case 'VALIDATE_CONFIG': {
        const config = payload as AmigurumiConfig;
        const result = validate_config(JSON.stringify(config));
        self.postMessage({ id, type: 'SUCCESS', payload: result });
        break;
      }

      default:
        throw new Error(`Unknown action type: ${type}`);
    }
  } catch (error) {
    self.postMessage({ 
      id, 
      type: 'ERROR', 
      payload: (error as Error).message 
    });
  }
};
