import React from 'react';

export interface SkeletonProps {
  width?: string | number;
  height?: string | number;
  circle?: boolean;
  className?: string;
}

export const Skeleton: React.FC<SkeletonProps> = ({
  width = '100%',
  height = '20px',
  circle = false,
  className = '',
}) => {
  const style: React.CSSProperties = {
    width,
    height,
    borderRadius: circle ? '50%' : 'var(--radius)',
    backgroundColor: 'var(--color-gray-light)',
    position: 'relative',
    overflow: 'hidden',
  };

  return (
    <div className={`shimmer ${className}`} style={style} aria-busy="true" aria-live="polite">
      <span className="sr-only">Loading...</span>
    </div>
  );
};
