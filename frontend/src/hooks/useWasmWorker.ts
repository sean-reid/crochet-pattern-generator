import { useRef, useEffect, useCallback, useState } from 'react';
import type { CrochetConfig } from '../types/config';
import type { MeshInfo } from '../types/mesh';
import type { CrochetPattern } from '../types/pattern';
import type { WorkerRequest, WorkerResponse, WorkerProgressType } from '../workers/wasm.worker';

let workerInstance: Worker | null = null;
let messageIdCounter = 0;

interface PendingRequest {
  resolve: (value: any) => void;
  reject: (error: Error) => void;
}

const pendingRequests = new Map<string, PendingRequest>();

export interface WorkerProgress {
  stage: WorkerProgressType;
  progress: number;
  message: string;
}

export const useWasmWorker = () => {
  const workerRef = useRef<Worker | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);
  const [isInitializing, setIsInitializing] = useState(false);
  const [initError, setInitError] = useState<string | null>(null);
  const [progress, setProgress] = useState<WorkerProgress | null>(null);

  // Initialize worker
  useEffect(() => {
    if (workerInstance) {
      workerRef.current = workerInstance;
      setIsInitialized(true);
      return;
    }

    try {
      // Create worker instance
      const worker = new Worker(
        new URL('../workers/wasm.worker.ts', import.meta.url),
        { type: 'module' }
      );

      workerRef.current = worker;
      workerInstance = worker;

      // Handle messages from worker
      worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
        const { id, type, payload, error } = event.data;

        if (type === 'PROGRESS' && payload?.progress) {
          setProgress(payload.progress);
          return;
        }

        const pending = pendingRequests.get(id);
        if (!pending) return;

        pendingRequests.delete(id);

        if (type === 'SUCCESS') {
          pending.resolve(payload);
        } else if (type === 'ERROR') {
          pending.reject(new Error(error || 'Unknown worker error'));
        }
      };

      // Handle worker errors
      worker.onerror = (error) => {
        console.error('Worker error:', error);
        setInitError(error.message);
      };

    } catch (error) {
      console.error('Failed to create worker:', error);
      setInitError(error instanceof Error ? error.message : 'Failed to create worker');
    }

    return () => {
      // Don't terminate worker on unmount - keep it alive for app lifetime
      // worker.terminate();
    };
  }, []);

  // Initialize WASM in worker
  const initializeWasm = useCallback(async () => {
    if (isInitialized || isInitializing) return;
    if (!workerRef.current) throw new Error('Worker not initialized');

    setIsInitializing(true);
    setInitError(null);

    try {
      await sendMessage('INIT_WASM', {});
      setIsInitialized(true);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to initialize WASM';
      setInitError(errorMessage);
      throw error;
    } finally {
      setIsInitializing(false);
    }
  }, [isInitialized, isInitializing]);

  // Send message to worker and wait for response
  const sendMessage = useCallback(
    <T = any>(type: WorkerRequest['type'], payload: WorkerRequest['payload'] = {}): Promise<T> => {
      return new Promise((resolve, reject) => {
        if (!workerRef.current) {
          reject(new Error('Worker not initialized'));
          return;
        }

        const id = `msg_${++messageIdCounter}`;
        pendingRequests.set(id, { resolve, reject });

        const message: WorkerRequest = { id, type, payload };
        workerRef.current.postMessage(message);

        // Timeout after 5 minutes
        setTimeout(() => {
          if (pendingRequests.has(id)) {
            pendingRequests.delete(id);
            reject(new Error('Worker request timeout'));
          }
        }, 5 * 60 * 1000);
      });
    },
    []
  );

  // Load mesh
  const loadMesh = useCallback(
    async (glbData: Uint8Array): Promise<MeshInfo> => {
      if (!isInitialized) {
        await initializeWasm();
      }

      const result = await sendMessage<{ meshInfo: MeshInfo }>('LOAD_MESH', { glbData });
      return result.meshInfo;
    },
    [isInitialized, initializeWasm, sendMessage]
  );

  // Generate pattern
  const generatePattern = useCallback(
    async (config: CrochetConfig): Promise<CrochetPattern> => {
      if (!isInitialized) {
        throw new Error('WASM not initialized');
      }

      setProgress({ stage: 'parameterization', progress: 0, message: 'Starting pattern generation...' });

      const result = await sendMessage<{ pattern: CrochetPattern }>('GENERATE_PATTERN', { config });
      
      setProgress(null);
      return result.pattern;
    },
    [isInitialized, sendMessage]
  );

  return {
    isInitialized,
    isInitializing,
    initError,
    progress,
    initializeWasm,
    loadMesh,
    generatePattern,
  };
};
