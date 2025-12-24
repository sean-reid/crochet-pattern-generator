export type YarnWeight = 'lace' | 'fingering' | 'sport' | 'worsted' | 'bulky';
export type StitchType = 'sc' | 'hdc' | 'dc';
export type ConstructionType = 'flat' | 'amigurumi';

// Frontend config interface (camelCase for TypeScript)
export interface CrochetConfig {
  gauge: {
    stitchesPerInch: number;
    rowsPerInch: number;
  };
  yarn: {
    weight: YarnWeight;
    hookSize: string;
  };
  construction: {
    type: ConstructionType;
    stitchTypes: StitchType[];
  };
  optimization: {
    maxDistortion: number;
    simplifyMesh: boolean;
    targetStitchCount?: number;
  };
}

// WASM API config interface (snake_case to match Rust)
export interface WasmCrochetConfig {
  stitches_per_inch: number;
  rows_per_inch: number;
  yarn_weight: string;
  hook_size_mm: number;
  target_width_inches?: number;
  target_height_inches?: number;
  construction_type: string;
  max_distortion: number;
  simplify_mesh: boolean;
  target_stitch_count?: number;
}

// Conversion function from frontend config to WASM config
export const toWasmConfig = (config: CrochetConfig): WasmCrochetConfig => {
  // Extract hook size in mm from string like "5.0mm (H/8)"
  const hookSizeMm = parseFloat(config.yarn.hookSize);

  return {
    stitches_per_inch: config.gauge.stitchesPerInch,
    rows_per_inch: config.gauge.rowsPerInch,
    yarn_weight: config.yarn.weight,
    hook_size_mm: isNaN(hookSizeMm) ? 5.0 : hookSizeMm,
    construction_type: config.construction.type,
    max_distortion: config.optimization.maxDistortion,
    simplify_mesh: config.optimization.simplifyMesh,
    target_stitch_count: config.optimization.targetStitchCount,
  };
};

// Default configurations
export const DEFAULT_CONFIG: CrochetConfig = {
  gauge: {
    stitchesPerInch: 5,
    rowsPerInch: 5,
  },
  yarn: {
    weight: 'worsted',
    hookSize: '5.0mm (H/8)',
  },
  construction: {
    type: 'flat',
    stitchTypes: ['sc', 'hdc', 'dc'],
  },
  optimization: {
    maxDistortion: 0.2,
    simplifyMesh: false,
  },
};

// Hook size reference data
export const HOOK_SIZES: { metric: string; us: string }[] = [
  { metric: '2.0mm', us: 'B/1' },
  { metric: '2.25mm', us: 'C/2' },
  { metric: '2.5mm', us: 'D/3' },
  { metric: '2.75mm', us: 'E/4' },
  { metric: '3.0mm', us: 'F/5' },
  { metric: '3.25mm', us: 'G/6' },
  { metric: '3.5mm', us: '7' },
  { metric: '3.75mm', us: 'H/8' },
  { metric: '4.0mm', us: 'I/9' },
  { metric: '4.5mm', us: 'J/10' },
  { metric: '5.0mm', us: 'K/10.5' },
  { metric: '5.5mm', us: 'L/11' },
  { metric: '6.0mm', us: 'M/13' },
  { metric: '6.5mm', us: 'N/15' },
  { metric: '8.0mm', us: 'P/16' },
  { metric: '9.0mm', us: 'Q' },
  { metric: '10.0mm', us: 'S' },
];

// Yarn weight typical gauge ranges
export const YARN_WEIGHT_GAUGES: Record<YarnWeight, { min: number; max: number }> = {
  lace: { min: 8, max: 10 },
  fingering: { min: 6, max: 8 },
  sport: { min: 5, max: 6 },
  worsted: { min: 4, max: 5 },
  bulky: { min: 2.5, max: 3.5 },
};
