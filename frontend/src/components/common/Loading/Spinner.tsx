import React from 'react';

export interface SpinnerProps {
  size?: 'small' | 'medium' | 'large';
  className?: string;
}

export const Spinner: React.FC<SpinnerProps> = ({ size = 'medium', className = '' }) => {
  const sizeMap = {
    small: 16,
    medium: 24,
    large: 32,
  };

  const spinnerSize = sizeMap[size];

  return (
    <div
      className={className}
      style={{
        width: spinnerSize,
        height: spinnerSize,
        border: `2px solid currentColor`,
        borderRightColor: 'transparent',
        borderRadius: '50%',
        animation: 'spin 0.6s linear infinite',
      }}
      role="status"
      aria-label="Loading"
    />
  );
};
