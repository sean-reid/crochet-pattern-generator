import type { Vector3, BoundingBox } from '../types/mesh';
import type { CrochetConfig } from '../types/config';

export const formatDimensions = (bbox: BoundingBox): string => {
  const width = bbox.max.x - bbox.min.x;
  const height = bbox.max.y - bbox.min.y;
  const depth = bbox.max.z - bbox.min.z;

  return `${width.toFixed(2)} × ${height.toFixed(2)} × ${depth.toFixed(2)} units`;
};

export const formatVector3 = (vec: Vector3, decimals = 2): string => {
  return `(${vec.x.toFixed(decimals)}, ${vec.y.toFixed(decimals)}, ${vec.z.toFixed(decimals)})`;
};

export const formatStitchCount = (count: number): string => {
  if (count === 1) return '1 stitch';
  return `${count.toLocaleString()} stitches`;
};

export const formatRowCount = (count: number): string => {
  if (count === 1) return '1 row';
  return `${count.toLocaleString()} rows`;
};

export const formatTime = (minutes: number): string => {
  if (minutes < 60) {
    return `${Math.round(minutes)} minutes`;
  }
  
  const hours = Math.floor(minutes / 60);
  const remainingMinutes = Math.round(minutes % 60);
  
  if (remainingMinutes === 0) {
    return `${hours} ${hours === 1 ? 'hour' : 'hours'}`;
  }
  
  return `${hours} ${hours === 1 ? 'hour' : 'hours'} ${remainingMinutes} ${remainingMinutes === 1 ? 'minute' : 'minutes'}`;
};

export const estimateCrochetTime = (stitchCount: number): number => {
  // Rough estimate: average 20-30 stitches per minute for experienced crocheter
  const stitchesPerMinute = 25;
  return stitchCount / stitchesPerMinute;
};

export const formatYarnEstimate = (
  stitchCount: number,
  config: CrochetConfig
): string => {
  // Rough estimate: ~7 yards per square inch for worsted weight
  const yardsPerSquareInch = 7;
  const stitchesPerSquareInch = config.gauge.stitchesPerInch * config.gauge.rowsPerInch;
  const squareInches = stitchCount / stitchesPerSquareInch;
  const yards = Math.ceil(squareInches * yardsPerSquareInch);
  
  if (yards < 100) {
    return `${yards} yards`;
  }
  
  // Convert to skeins (typical skein is 220 yards for worsted weight)
  const skeins = Math.ceil(yards / 220);
  return `${skeins} ${skeins === 1 ? 'skein' : 'skeins'} (~${yards} yards)`;
};

export const formatGauge = (config: CrochetConfig): string => {
  return `${config.gauge.stitchesPerInch} sts × ${config.gauge.rowsPerInch} rows = 1 inch`;
};

export const formatPercentage = (value: number, decimals = 0): string => {
  return `${(value * 100).toFixed(decimals)}%`;
};

export const formatNumber = (value: number, decimals = 0): string => {
  return value.toLocaleString(undefined, {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  });
};

export const capitalizeFirst = (str: string): string => {
  return str.charAt(0).toUpperCase() + str.slice(1);
};

export const pluralize = (count: number, singular: string, plural?: string): string => {
  if (count === 1) return `${count} ${singular}`;
  return `${count} ${plural || singular + 's'}`;
};
