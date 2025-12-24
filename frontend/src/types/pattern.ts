import type { Vector2, Vector3 } from './mesh';
import type { StitchType } from './config';

export interface PatternMetadata {
  stitchCount: number;
  rowCount: number;
  estimatedTime: string;
  yarnEstimate: string;
  dimensions: {
    width: number;
    height: number;
    depth: number;
  };
}

export interface Stitch {
  id: number;
  type: StitchType | 'inc' | 'dec';
  position3D: Vector3;
  position2D: Vector2;
  row: number;
  connections: number[]; // IDs of connected stitches
}

export interface RowInstruction {
  count: number;
  type: string;
  instruction: string;
}

export interface Row {
  number: number;
  stitches: RowInstruction[];
  totalStitches: number;
}

export interface Instructions {
  rows: Row[];
}

export interface Diagram {
  svg: string; // SVG markup
  stitchMap: Map<number, Vector2>; // Stitch ID to diagram coords
}

export interface CrochetPattern {
  metadata: PatternMetadata;
  stitches: Stitch[];
  instructions: Instructions;
  diagram: Diagram;
}

export interface PatternGenerationProgress {
  stage: 'parameterization' | 'grid-generation' | 'stitch-classification' | 'optimization' | 'instruction-generation' | 'complete';
  progress: number; // 0-100
  message: string;
}

export interface PatternGenerationResult {
  success: boolean;
  pattern?: CrochetPattern;
  error?: string;
}

// Stitch abbreviation mappings
export const STITCH_ABBREVIATIONS: Record<string, string> = {
  'sc': 'single crochet',
  'hdc': 'half double crochet',
  'dc': 'double crochet',
  'inc': 'increase',
  'dec': 'decrease',
  'ch': 'chain',
  'sl st': 'slip stitch',
  'mr': 'magic ring',
};

// Stitch symbols for diagrams
export const STITCH_SYMBOLS: Record<string, string> = {
  'sc': '×',
  'hdc': 'T',
  'dc': '⊤',
  'inc': '⋀',
  'dec': '⋁',
  'ch': '○',
  'sl st': '•',
};
