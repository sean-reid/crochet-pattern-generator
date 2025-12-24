import { useState, useEffect } from 'react';
import type { WasmModule, WasmLoadState } from '../types/wasm';

let wasmModuleInstance: WasmModule | null = null;

export const useWasmModule = () => {
  const [loadState, setLoadState] = useState<WasmLoadState>({
    loading: true,
    loaded: false,
    error: undefined,
  });

  useEffect(() => {
    const loadWasm = async () => {
      try {
        // In production, this would import the actual WASM module
        // import init, { MeshProcessor } from '../wasm/crochet_pattern_wasm.js';
        // const wasm = await init();
        
        // For now, we're just setting up the interface
        // The actual WASM module should be built and placed in public/wasm/
        
        // Simulate WASM loading
        // In production: wasmModuleInstance = wasm;
        
        setLoadState({
          loading: false,
          loaded: true,
        });
      } catch (error) {
        console.error('Failed to load WASM module:', error);
        setLoadState({
          loading: false,
          loaded: false,
          error: error instanceof Error ? error.message : 'Failed to load WASM module',
        });
      }
    };

    if (!wasmModuleInstance) {
      loadWasm();
    } else {
      setLoadState({
        loading: false,
        loaded: true,
      });
    }
  }, []);

  return { wasmModule: wasmModuleInstance, ...loadState };
};
