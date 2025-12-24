import { useState, useCallback } from 'react';
import { useWasmWorker } from './useWasmWorker';
import type { MeshInfo, ModelFile } from '../types/mesh';
import type { CrochetConfig } from '../types/config';
import type { CrochetPattern } from '../types/pattern';
import { MeshLoadError, PatternGenerationError } from '../types/wasm';

/**
 * ⚡ WASM Processor with Web Worker Support (ENABLED BY DEFAULT)
 * 
 * This hook runs all WASM processing in a separate Web Worker thread for:
 * - Non-blocking UI during heavy computations
 * - True parallelism with separate CPU thread
 * - Smooth progress indicators
 * - Better memory management
 * 
 * To disable Web Worker and use main thread (not recommended):
 * Set USE_WEB_WORKER = false below
 */

// ⚡ Feature flag - Web Worker enabled by default for best performance
const USE_WEB_WORKER = true;

export const useWasmProcessorV2 = () => {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Web Worker implementation
  const worker = useWasmWorker();

  const loadMesh = useCallback(
    async (modelFile: ModelFile): Promise<MeshInfo | null> => {
      setIsLoading(true);
      setError(null);

      try {
        const glbData = new Uint8Array(modelFile.data);

        if (USE_WEB_WORKER) {
          // Auto-initialize WASM if not already initialized
          if (!worker.isInitialized && !worker.isInitializing) {
            await worker.initializeWasm();
          }
          
          // Use Web Worker (runs in separate thread)
          const meshInfo = await worker.loadMesh(glbData);
          return meshInfo;
        } else {
          // Direct WASM call (blocks main thread)
          throw new MeshLoadError(
            `WASM module not loaded. Cannot process ${glbData.length} bytes. Please build the Rust backend and place the compiled WASM module in public/wasm/`
          );
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Failed to load mesh';
        setError(errorMessage);
        console.error('Mesh loading error:', err);
        return null;
      } finally {
        setIsLoading(false);
      }
    },
    [worker]
  );

  const generatePattern = useCallback(
    async (config: CrochetConfig): Promise<CrochetPattern | null> => {
      setIsLoading(true);
      setError(null);

      try {
        if (USE_WEB_WORKER) {
          // Auto-initialize WASM if not already initialized
          if (!worker.isInitialized && !worker.isInitializing) {
            await worker.initializeWasm();
          }
          
          // Use Web Worker (runs in separate thread)
          const pattern = await worker.generatePattern(config);
          return pattern;
        } else {
          // Direct WASM call (blocks main thread)
          throw new PatternGenerationError(
            `WASM module not loaded. Cannot generate pattern with ${config.gauge.stitchesPerInch} stitches/inch gauge. Please build the Rust backend and place the compiled WASM module in public/wasm/`
          );
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Failed to generate pattern';
        setError(errorMessage);
        console.error('Pattern generation error:', err);
        return null;
      } finally {
        setIsLoading(false);
      }
    },
    [worker]
  );

  const cleanup = useCallback(async () => {
    if (USE_WEB_WORKER) {
      // No cleanup needed - WASM functions are stateless
      // The worker keeps the model data until new mesh is loaded
    }
  }, [worker]);

  return {
    loadMesh,
    generatePattern,
    cleanup,
    isLoading,
    error,
    progress: worker.progress,
    workerStatus: {
      isInitialized: worker.isInitialized,
      isInitializing: worker.isInitializing,
      initError: worker.initError,
    },
  };
};
