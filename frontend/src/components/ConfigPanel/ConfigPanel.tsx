import React from 'react';
import { useConfig } from '../../context/ConfigContext';
import { useApp } from '../../context/AppContext';
import { useWasmProcessorV2 } from '../../hooks/useWasmProcessorV2';
import { Button } from '../common/Button';
import { Input } from '../common/Input';
import { Card } from '../common/Card';
import { ProgressBar } from '../common/Loading';
import { HOOK_SIZES } from '../../types/config';
import type { YarnWeight, ConstructionType } from '../../types/config';

const ConfigPanel: React.FC = () => {
  const { config, updateConfig } = useConfig();
  const { modelFile, setPattern, setIsGenerating } = useApp();
  const { generatePattern: generatePatternWorker, progress: workerProgress, isLoading } = useWasmProcessorV2();

  const handleGeneratePattern = async () => {
    if (!modelFile) return;
    
    setIsGenerating(true);
    
    // Use Web Worker version for non-blocking pattern generation
    const pattern = await generatePatternWorker(config);
    
    if (pattern) {
      setPattern(pattern);
    } else {
      console.error('Pattern generation failed');
      alert('Failed to generate pattern. Please check console for details.');
    }
    
    setIsGenerating(false);
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-16)' }}>
      <Card>
        <h3 style={{ marginBottom: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', fontWeight: 'var(--font-weight-semibold)' }}>Gauge</h3>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-12)' }}>
          <Input
            label="Stitches per inch"
            type="number"
            value={config.gauge.stitchesPerInch}
            onChange={(e) => updateConfig({ gauge: { ...config.gauge, stitchesPerInch: Number(e.target.value) } })}
            min={1}
            max={20}
          />
          <Input
            label="Rows per inch"
            type="number"
            value={config.gauge.rowsPerInch}
            onChange={(e) => updateConfig({ gauge: { ...config.gauge, rowsPerInch: Number(e.target.value) } })}
            min={1}
            max={20}
          />
        </div>
      </Card>

      <Card>
        <h3 style={{ marginBottom: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', fontWeight: 'var(--font-weight-semibold)' }}>Yarn</h3>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-12)' }}>
          <div>
            <label style={{ display: 'block', fontSize: 'var(--font-size-sm)', fontWeight: 'var(--font-weight-medium)', marginBottom: 'var(--spacing-8)' }}>
              Weight
            </label>
            <select
              value={config.yarn.weight}
              onChange={(e) => updateConfig({ yarn: { ...config.yarn, weight: e.target.value as YarnWeight } })}
              style={{ width: '100%', padding: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', border: '1px solid var(--color-gray-pale)', borderRadius: 'var(--radius)', fontFamily: 'var(--font-sans)' }}
            >
              <option value="lace">Lace</option>
              <option value="fingering">Fingering</option>
              <option value="sport">Sport</option>
              <option value="worsted">Worsted</option>
              <option value="bulky">Bulky</option>
            </select>
          </div>
          <div>
            <label style={{ display: 'block', fontSize: 'var(--font-size-sm)', fontWeight: 'var(--font-weight-medium)', marginBottom: 'var(--spacing-8)' }}>
              Hook Size
            </label>
            <select
              value={config.yarn.hookSize}
              onChange={(e) => updateConfig({ yarn: { ...config.yarn, hookSize: e.target.value } })}
              style={{ width: '100%', padding: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', border: '1px solid var(--color-gray-pale)', borderRadius: 'var(--radius)', fontFamily: 'var(--font-sans)' }}
            >
              {HOOK_SIZES.map((size) => (
                <option key={size.metric} value={`${size.metric} (${size.us})`}>
                  {size.metric} ({size.us})
                </option>
              ))}
            </select>
          </div>
        </div>
      </Card>

      <Card>
        <h3 style={{ marginBottom: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', fontWeight: 'var(--font-weight-semibold)' }}>Construction</h3>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-12)' }}>
          <div>
            <label style={{ display: 'block', fontSize: 'var(--font-size-sm)', fontWeight: 'var(--font-weight-medium)', marginBottom: 'var(--spacing-8)' }}>
              Type
            </label>
            <select
              value={config.construction.type}
              onChange={(e) => updateConfig({ construction: { ...config.construction, type: e.target.value as ConstructionType } })}
              style={{ width: '100%', padding: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', border: '1px solid var(--color-gray-pale)', borderRadius: 'var(--radius)', fontFamily: 'var(--font-sans)' }}
            >
              <option value="flat">Flat</option>
              <option value="amigurumi">Amigurumi (in-the-round)</option>
            </select>
          </div>
        </div>
      </Card>

      {workerProgress && (
        <Card>
          <ProgressBar
            progress={workerProgress.progress}
            label={workerProgress.message}
            showPercentage
          />
        </Card>
      )}

      <Button
        variant="primary"
        size="large"
        fullWidth
        onClick={handleGeneratePattern}
        disabled={!modelFile || isLoading}
        loading={isLoading}
      >
        Generate Pattern
      </Button>
    </div>
  );
};

export default ConfigPanel;
