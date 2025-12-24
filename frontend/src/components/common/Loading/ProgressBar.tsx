import React from 'react';

export interface ProgressBarProps {
  progress: number; // 0-100
  label?: string;
  showPercentage?: boolean;
  className?: string;
}

export const ProgressBar: React.FC<ProgressBarProps> = ({
  progress,
  label,
  showPercentage = true,
  className = '',
}) => {
  const clampedProgress = Math.max(0, Math.min(100, progress));

  return (
    <div className={className}>
      {(label || showPercentage) && (
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 'var(--spacing-8)' }}>
          {label && <span style={{ fontSize: 'var(--font-size-sm)', color: 'var(--color-gray-medium)' }}>{label}</span>}
          {showPercentage && <span style={{ fontSize: 'var(--font-size-sm)', fontWeight: 'var(--font-weight-medium)' }}>{Math.round(clampedProgress)}%</span>}
        </div>
      )}
      <div
        style={{
          width: '100%',
          height: '4px',
          backgroundColor: 'var(--color-gray-pale)',
          borderRadius: 'var(--radius-full)',
          overflow: 'hidden',
        }}
        role="progressbar"
        aria-valuenow={clampedProgress}
        aria-valuemin={0}
        aria-valuemax={100}
      >
        <div
          style={{
            width: `${clampedProgress}%`,
            height: '100%',
            backgroundColor: 'var(--color-terracotta)',
            transition: 'width var(--transition-slow) var(--easing-out)',
          }}
        />
      </div>
    </div>
  );
};
