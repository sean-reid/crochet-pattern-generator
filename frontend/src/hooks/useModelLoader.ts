import { useCallback } from 'react';
import { useApp } from '../context/AppContext';
import { useWasmProcessorV2 } from './useWasmProcessorV2';
import { validateFile, readFileAsArrayBuffer } from '../utils/fileValidation';
// Ensure Vector3 and MeshInfo are imported for type safety
import type { ModelFile, MeshInfo, Vector3 } from '../types/mesh';

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

        // Create initial ModelFile object
        const modelFile: ModelFile = {
          name: file.name,
          size: file.size,
          data: arrayBuffer,
        };

        // 1. Call WASM processor. We cast to 'any' because the WASM 
        // currently returns the raw Rust-serialized structure.
        const rawResult = await loadMesh(modelFile) as any;

        // 2. Check for success flag returned by bindings.rs
        if (!rawResult || rawResult.success === false) {
          return {
            success: false,
            error: rawResult?.error || 'Failed to load mesh information',
          };
        }

        // 3. Helper to convert Rust array [x, y, z] to Vector3 {x, y, z}
        const toVector3 = (arr: [number, number, number] | undefined): Vector3 => ({
          x: arr?.[0] ?? 0,
          y: arr?.[1] ?? 0,
          z: arr?.[2] ?? 0
        });

        // 4. Transform raw WASM keys to match the MeshInfo interface
        const transformedMeshInfo: MeshInfo = {
          vertexCount: rawResult.vertices ?? 0, // Map 'vertices' to 'vertexCount'
          faceCount: rawResult.faces ?? 0,      // Map 'faces' to 'faceCount'
          boundingBox: {
            // Safely access bounds to avoid "undefined is not an object"
            min: toVector3(rawResult.bounds?.min),
            max: toVector3(rawResult.bounds?.max),
          },
          // Populate mandatory fields with defaults as required by mesh.ts
          surfaceArea: 0,
          isManifold: true,
          hasUVs: false,
          hasNormals: false,
        };

        // 5. Finalize the model file and update global state
        modelFile.meshInfo = transformedMeshInfo;
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
