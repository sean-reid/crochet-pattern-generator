import { useState, useEffect } from 'react';
import { wrap } from 'comlink';
import type { AmigurumiConfig, ProfileCurve, CrochetPattern } from '../types';
import type { PatternGeneratorAPI } from '../workers/pattern-worker';

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
  const [worker, setWorker] = useState<PatternGeneratorAPI | null>(null);

  useEffect(() => {
    const patternWorker = new Worker(
      new URL('../workers/pattern-worker.ts', import.meta.url),
      { type: 'module' }
    );
    const api = wrap<PatternGeneratorAPI>(patternWorker);
    setWorker(api);

    return () => {
      patternWorker.terminate();
    };
  }, []);

  const handleInputChange = (field: keyof AmigurumiConfig, value: number) => {
    onChange({ ...config, [field]: value });
  };

  const handleYarnChange = (field: string, value: number) => {
    onChange({
      ...config,
      yarn: { ...config.yarn, [field]: value },
    });
  };

  const handleGenerate = async () => {
    if (!profile) {
      onError('Please draw a profile first');
      return;
    }

    if (!worker) {
      onError('Pattern generator not initialized');
      return;
    }

    setIsGenerating(true);

    try {
      const pattern = await worker.generatePattern(profile, config);
      onGeneratePattern(pattern);
    } catch (error) {
      onError(`Failed to generate pattern: ${error}`);
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
          </div>

          <div>
            <label className="label">Start Diameter (cm)</label>
            <input
              type="number"
              value={config.start_diameter_cm}
              onChange={(e) =>
                handleInputChange('start_diameter_cm', parseFloat(e.target.value))
              }
              className="input"
              min="0"
              max="50"
              step="0.5"
            />
          </div>

          <div>
            <label className="label">End Diameter (cm)</label>
            <input
              type="number"
              value={config.end_diameter_cm}
              onChange={(e) =>
                handleInputChange('end_diameter_cm', parseFloat(e.target.value))
              }
              className="input"
              min="0"
              max="50"
              step="0.5"
            />
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
            Please draw a profile in the Draw tab first
          </p>
        )}
      </div>
    </div>
  );
}
