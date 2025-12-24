import { useState, useCallback } from 'react';
import { useWasmWorker } from './useWasmWorker';
import type { MeshInfo, ModelFile, Vector3 } from '../types/mesh';
import type { CrochetConfig } from '../types/config';
import type { CrochetPattern, Stitch } from '../types/pattern';
import { MeshLoadError, PatternGenerationError } from '../types/wasm';

/**
 * ⚡ WASM Processor with Web Worker Support (ENABLED BY DEFAULT)
 */

const USE_WEB_WORKER = true;

export const useWasmProcessorV2 = () => {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const worker = useWasmWorker();

  /**
   * 🔄 Mesh Transformation: Maps Rust snake_case and arrays to Frontend MeshInfo
   */
  const transformMeshInfo = (raw: any): MeshInfo => {
    const toVector3 = (arr: [number, number, number] | undefined): Vector3 => ({
      x: arr?.[0] ?? 0,
      y: arr?.[1] ?? 0,
      z: arr?.[2] ?? 0
    });

    return {
      vertexCount: raw.vertices ?? 0, // Map 'vertices' -> 'vertexCount'
      faceCount: raw.faces ?? 0,    // Map 'faces' -> 'faceCount'
      boundingBox: {
        min: toVector3(raw.bounds?.min),
        max: toVector3(raw.bounds?.max)
      },
      surfaceArea: raw.surface_area ?? 0,
      isManifold: raw.is_manifold ?? true,
      hasUVs: raw.has_uvs ?? false,
      hasNormals: raw.has_normals ?? false
    };
  };

  /**
   * 🔄 Pattern Transformation: Maps Rust results to camelCase CrochetPattern interface
   */
  const transformPattern = (raw: any): CrochetPattern => {
    return {
      metadata: {
        stitchCount: raw.metadata.stitch_count, // Map 'stitch_count' -> 'stitchCount'
        rowCount: raw.metadata.row_count,       // Map 'row_count' -> 'rowCount'
        estimatedTime: raw.metadata.estimated_time,
        yarnEstimate: raw.metadata.yarn_estimate,
        dimensions: raw.metadata.dimensions
      },
      stitches: raw.stitches.map((s: any): Stitch => ({
        id: s.id,
        // Map Rust Enum variants to frontend abbreviations used in pattern.ts
        type: s.stitch_type === 'Increase' ? 'inc' : 
              s.stitch_type === 'Decrease' ? 'dec' : 'sc',
        position3D: { x: s.position_3d[0], y: s.position_3d[1], z: s.position_3d[2] },
        position2D: { x: s.position_2d[0], y: s.position_2d[1] },
        row: s.row,
        connections: s.connections
      })),
      instructions: {
        rows: raw.instructions.rows.map((r: any) => ({
          number: r.number,
          totalStitches: r.total_stitches,
          stitches: r.stitches.map((sg: any) => ({
            count: sg.count,
            type: sg.stitch_type,
            instruction: sg.instruction
          }))
        }))
      },
      diagram: {
        svg: raw.diagram || '',
        stitchMap: new Map()
      }
    };
  };

  const loadMesh = useCallback(
    async (modelFile: ModelFile): Promise<MeshInfo | null> => {
      setIsLoading(true);
      setError(null);

      try {
        const glbData = new Uint8Array(modelFile.data);

        if (USE_WEB_WORKER) {
          if (!worker.isInitialized && !worker.isInitializing) {
            await worker.initializeWasm();
          }
          
          const rawMeshInfo = await worker.loadMesh(glbData);
          if (!rawMeshInfo) return null;

          // Apply transformation before returning to the UI
          return transformMeshInfo(rawMeshInfo);
        } else {
          throw new MeshLoadError("Direct WASM call not implemented.");
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
          if (!worker.isInitialized && !worker.isInitializing) {
            await worker.initializeWasm();
          }
          
          const result = await worker.generatePattern(config) as any;
          
          if (!result || !result.success || !result.pattern) {
            throw new Error(result?.error || 'Failed to generate pattern');
          }

          // Transform raw pattern data to resolve 'stitchCount' evaluation error
          return transformPattern(result.pattern);
        } else {
          throw new PatternGenerationError("WASM Worker required for pattern generation.");
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
    // WASM functions are stateless; worker maintains data until new load
  }, []);

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
