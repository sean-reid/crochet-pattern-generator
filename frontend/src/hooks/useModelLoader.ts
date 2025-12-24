import { useCallback } from 'react';
import { useApp } from '../context/AppContext';
import { useWasmProcessorV2 } from './useWasmProcessorV2';
import { validateFile, readFileAsArrayBuffer } from '../utils/fileValidation';
import type { ModelFile } from '../types/mesh';

export const useModelLoader = () => {
  const { setModelFile } = useApp();
  const { loadMesh, cleanup } = useWasmProcessorV2();

  const loadModel = useCallback(
    async (file: File): Promise<{ success: boolean; error?: string }> => {
      // Validate file
      const validation = validateFile(file);
      if (!validation.valid) {
        return {
          success: false,
          error: validation.errors[0].message,
        };
      }

      try {
        // Read file as ArrayBuffer
        const arrayBuffer = await readFileAsArrayBuffer(file);

        // Create ModelFile object
        const modelFile: ModelFile = {
          name: file.name,
          size: file.size,
          data: arrayBuffer,
        };

        // Load mesh through WASM processor (Web Worker) to get mesh info
        const meshInfo = await loadMesh(modelFile);

        if (!meshInfo) {
          return {
            success: false,
            error: 'Failed to load mesh information from file',
          };
        }

        // Update model file with mesh info
        modelFile.meshInfo = meshInfo;

        // Store in app state
        setModelFile(modelFile);

        return { success: true };
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Failed to load model';
        return {
          success: false,
          error: errorMessage,
        };
      }
    },
    [loadMesh, setModelFile]
  );

  const clearModel = useCallback(() => {
    cleanup();
    setModelFile(null);
  }, [cleanup, setModelFile]);

  return {
    loadModel,
    clearModel,
  };
};
