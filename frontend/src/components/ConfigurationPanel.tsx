import { useState, useEffect, useRef } from 'react';
import type { AmigurumiConfig, ProfileCurve, CrochetPattern } from '../types';

interface Props {
  config: AmigurumiConfig;
  profile: ProfileCurve | null;
  onChange: (config: AmigurumiConfig) => void;
  onGeneratePattern: (pattern: CrochetPattern) => void;
  onError: (error: string) => void;
}

export default function ConfigurationPanel({
  config,
  profile,
  onChange,
  onGeneratePattern,
  onError,
}: Props) {
  const [isGenerating, setIsGenerating] = useState(false);
  const workerRef = useRef<Worker | null>(null);
  
  // Track pending promises to resolve them when the worker responds
  const pendingRequests = useRef<Map<string, { resolve: Function; reject: Function }>>(new Map());

  useEffect(() => {
    // Initialize the standard Web Worker
    const patternWorker = new Worker(
      new URL('../workers/pattern-worker.ts', import.meta.url),
      { type: 'module' }
    );

    // Set up response listener
    patternWorker.onmessage = (e) => {
      const { id, type, payload } = e.data;
      const request = pendingRequests.current.get(id);
      
      if (request) {
        if (type === 'SUCCESS') {
          request.resolve(payload);
        } else {
          request.reject(new Error(payload));
        }
        pendingRequests.current.delete(id);
      }
    };

    workerRef.current = patternWorker;

    return () => {
      patternWorker.terminate();
    };
  }, []);

  // Helper function to communicate with the worker via Promises
  const callWorker = (type: string, payload: any): Promise<any> => {
    return new Promise((resolve, reject) => {
      if (!workerRef.current) {
        return reject(new Error('Pattern generator worker not initialized'));
      }
      
      const id = Math.random().toString(36).substring(7);
      pendingRequests.current.set(id, { resolve, reject });
      workerRef.current.postMessage({ id, type, payload });
    });
  };

  const handleInputChange = (field: keyof AmigurumiConfig, value: number) => {
    onChange({ ...config, [field]: value });
  };

  const handleYarnChange = (field: string, value: number) => {
    onChange({
      ...config,
      yarn: { ...config.yarn, [field]: value as any },
    });
  };

  const handleGenerate = async () => {
    if (!profile) {
      onError('Please draw a profile first');
      return;
    }

    setIsGenerating(true);

    try {
      // Send the request to the worker
      const pattern = await callWorker('GENERATE_PATTERN', { profile, config });
      onGeneratePattern(pattern);
    } catch (error) {
      onError(`Failed to generate pattern: ${(error as Error).message}`);
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-6">
          Amigurumi Dimensions
        </h2>

        <div className="space-y-4">
          <div>
            <label className="label">Total Height (cm)</label>
            <input
              type="number"
              value={config.total_height_cm}
              onChange={(e) =>
                handleInputChange('total_height_cm', parseFloat(e.target.value))
              }
              className="input"
              min="1"
              max="100"
              step="0.5"
            />
            <p className="text-xs text-slate-500 mt-1">
              Height determines the number of rows in your pattern
            </p>
          </div>

          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
            <p className="text-sm text-slate-700">
              <strong>Note:</strong> Draw your desired shape and size directly on the canvas.
              The drawn profile defines both the shape and dimensions of your amigurumi.
            </p>
          </div>
        </div>
      </div>

      <div className="card p-8">
        <h2 className="text-xl font-semibold text-slate-900 mb-6">
          Yarn Specifications
        </h2>

        <div className="space-y-4">
          <div>
            <label className="label">Gauge - Stitches per cm</label>
            <input
              type="number"
              value={config.yarn.gauge_stitches_per_cm}
              onChange={(e) =>
                handleYarnChange('gauge_stitches_per_cm', parseFloat(e.target.value))
              }
              className="input"
              min="1"
              max="10"
              step="0.1"
            />
            <p className="text-xs text-slate-500 mt-1">
              Typical range: 2.5-4.0 for worsted weight
            </p>
          </div>

          <div>
            <label className="label">Gauge - Rows per cm</label>
            <input
              type="number"
              value={config.yarn.gauge_rows_per_cm}
              onChange={(e) =>
                handleYarnChange('gauge_rows_per_cm', parseFloat(e.target.value))
              }
              className="input"
              min="1"
              max="10"
              step="0.1"
            />
            <p className="text-xs text-slate-500 mt-1">
              Typical range: 2.5-4.0 for worsted weight
            </p>
          </div>

          <div>
            <label className="label">Hook Size (mm)</label>
            <input
              type="number"
              value={config.yarn.recommended_hook_size_mm}
              onChange={(e) =>
                handleYarnChange(
                  'recommended_hook_size_mm',
                  parseFloat(e.target.value)
                )
              }
              className="input"
              min="2"
              max="10"
              step="0.25"
            />
            <p className="text-xs text-slate-500 mt-1">
              Common sizes: 3.5mm (E), 4.0mm (G), 5.0mm (H)
            </p>
          </div>
        </div>
      </div>

      <div className="lg:col-span-2">
        <button
          onClick={handleGenerate}
          disabled={isGenerating || !profile}
          className={`w-full btn-primary ${
            isGenerating || !profile ? 'opacity-50 cursor-not-allowed' : ''
          }`}
        >
          {isGenerating ? 'Generating Pattern...' : 'Generate Pattern'}
        </button>

        {!profile && (
          <p className="text-center text-sm text-slate-600 mt-2">
            Please draw a profile in the Draw tab first. The profile defines the shape of your amigurumi.
          </p>
        )}
        
        {profile && (
          <p className="text-center text-xs text-slate-500 mt-2">
            Tip: Adjust dimensions and yarn specifications to match your desired size and materials.
          </p>
        )}
      </div>
    </div>
  );
}
