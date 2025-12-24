import React from 'react';
import { useApp } from '../../context/AppContext';
import { Card } from '../common/Card';
import { Skeleton } from '../common/Loading';
import { formatStitchCount, formatRowCount } from '../../utils/formatters';

const PatternPreview: React.FC = () => {
  const { pattern, isGenerating } = useApp();

  if (isGenerating) {
    return (
      <Card>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-16)' }}>
          <Skeleton height="40px" />
          <Skeleton height="100px" />
          <Skeleton height="60px" />
        </div>
      </Card>
    );
  }

  if (!pattern) {
    return (
      <Card style={{ backgroundColor: 'var(--color-off-white)', padding: 'var(--spacing-24)', textAlign: 'center' }}>
        <p style={{ color: 'var(--color-gray-medium)', fontSize: 'var(--font-size-sm)' }}>
          No pattern generated yet. Upload a model and click "Generate Pattern" to begin.
        </p>
      </Card>
    );
  }

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-16)' }}>
      <Card>
        <h3 style={{ marginBottom: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', fontWeight: 'var(--font-weight-semibold)' }}>Summary</h3>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-8)', fontSize: 'var(--font-size-sm)' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span style={{ color: 'var(--color-gray-medium)' }}>Total Stitches:</span>
            <span style={{ fontWeight: 'var(--font-weight-medium)', fontFamily: 'var(--font-mono)' }}>{formatStitchCount(pattern.metadata.stitchCount)}</span>
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span style={{ color: 'var(--color-gray-medium)' }}>Rows:</span>
            <span style={{ fontWeight: 'var(--font-weight-medium)', fontFamily: 'var(--font-mono)' }}>{formatRowCount(pattern.metadata.rowCount)}</span>
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span style={{ color: 'var(--color-gray-medium)' }}>Estimated Time:</span>
            <span style={{ fontWeight: 'var(--font-weight-medium)', fontFamily: 'var(--font-mono)' }}>{pattern.metadata.estimatedTime}</span>
          </div>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <span style={{ color: 'var(--color-gray-medium)' }}>Yarn Estimate:</span>
            <span style={{ fontWeight: 'var(--font-weight-medium)', fontFamily: 'var(--font-mono)' }}>{pattern.metadata.yarnEstimate}</span>
          </div>
        </div>
      </Card>

      <Card>
        <h3 style={{ marginBottom: 'var(--spacing-12)', fontSize: 'var(--font-size-base)', fontWeight: 'var(--font-weight-semibold)' }}>Dimensions</h3>
        <div style={{ display: 'flex', gap: 'var(--spacing-16)', fontSize: 'var(--font-size-sm)' }}>
          <div>
            <span style={{ color: 'var(--color-gray-medium)' }}>W: </span>
            <span style={{ fontWeight: 'var(--font-weight-medium)' }}>{pattern.metadata.dimensions.width}"</span>
          </div>
          <div>
            <span style={{ color: 'var(--color-gray-medium)' }}>H: </span>
            <span style={{ fontWeight: 'var(--font-weight-medium)' }}>{pattern.metadata.dimensions.height}"</span>
          </div>
          <div>
            <span style={{ color: 'var(--color-gray-medium)' }}>D: </span>
            <span style={{ fontWeight: 'var(--font-weight-medium)' }}>{pattern.metadata.dimensions.depth}"</span>
          </div>
        </div>
      </Card>
    </div>
  );
};

export default PatternPreview;
