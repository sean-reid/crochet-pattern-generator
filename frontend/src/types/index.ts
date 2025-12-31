export interface Point2D {
  x: number;
  y: number;
}

export interface SplineSegment {
  start: Point2D;
  control1: Point2D;
  control2: Point2D;
  end: Point2D;
}

export interface ProfileCurve {
  segments: SplineSegment[];
  start_radius: number;
  end_radius: number;
}

export interface YarnSpec {
  gauge_stitches_per_cm: number;
  gauge_rows_per_cm: number;
  recommended_hook_size_mm: number;
}

export interface AmigurumiConfig {
  total_height_cm: number;
  yarn: YarnSpec;
}

export type StitchType = 'SC' | 'INC' | 'DEC' | 'INVDEC';

export interface StitchInstruction {
  stitch_type: StitchType;
  angular_position: number;
  stitch_index: number;
}

export interface Row {
  row_number: number;
  total_stitches: number;
  pattern: StitchInstruction[];
}

export interface PatternMetadata {
  total_rows: number;
  total_stitches: number;
  estimated_time_minutes: number;
  yarn_length_meters: number;
}

export interface CrochetPattern {
  rows: Row[];
  metadata: PatternMetadata;
}

export interface ValidationError {
  field: string;
  message: string;
}

export type DrawingTool = 'select' | 'add' | 'delete';

export interface AppState {
  currentTab: 'draw' | 'configure' | 'preview' | 'export';
  profile: ProfileCurve | null;
  config: AmigurumiConfig;
  pattern: CrochetPattern | null;
  isGenerating: boolean;
  error: string | null;
  drawingTool: DrawingTool;
}
