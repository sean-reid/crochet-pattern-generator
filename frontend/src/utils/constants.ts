export const APP_NAME = 'Crochet Pattern Generator';
export const APP_VERSION = '0.1.0';

// File upload constraints
export const MAX_FILE_SIZE = 50 * 1024 * 1024; // 50MB
export const SUPPORTED_FILE_TYPES = ['.glb', '.gltf'];
export const ACCEPTED_MIME_TYPES = ['model/gltf-binary', 'model/gltf+json'];

// Pattern generation defaults
export const DEFAULT_TARGET_STITCH_COUNT = 5000;
export const MIN_STITCH_COUNT = 100;
export const MAX_STITCH_COUNT = 50000;

// UI constants
export const BREAKPOINTS = {
  mobile: 640,
  tablet: 768,
  desktop: 1024,
  wide: 1440,
} as const;

export const ANIMATION_DURATION = {
  fast: 150,
  base: 200,
  medium: 250,
  slow: 300,
  slower: 400,
} as const;

// 3D Viewer constants
export const CAMERA_DEFAULTS = {
  fov: 50,
  near: 0.1,
  far: 1000,
  position: [5, 5, 5] as [number, number, number],
} as const;

export const GRID_SIZE = 10;
export const GRID_DIVISIONS = 10;

// Color constants for 3D visualization
export const STITCH_COLORS = {
  normal: '#C67B5C', // terracotta
  increase: '#E89B87', // coral
  decrease: '#8BA888', // sage
  selected: '#FFFFFF', // white
  hover: '#FFEEE9', // coral-light
  guide: '#E8E8E8', // gray-pale
} as const;

// Export formats
export const EXPORT_FORMATS = {
  PDF: 'pdf',
  SVG: 'svg',
  JSON: 'json',
  CSV: 'csv',
} as const;

// Stitch instruction templates
export const INSTRUCTION_TEMPLATES = {
  magicRing: 'Make a magic ring',
  chain: (count: number) => `Ch ${count}`,
  singleCrochet: (count: number) => count === 1 ? 'Sc' : `${count} sc`,
  halfDoubleCrochet: (count: number) => count === 1 ? 'Hdc' : `${count} hdc`,
  doubleCrochet: (count: number) => count === 1 ? 'Dc' : `${count} dc`,
  increase: (count: number) => count === 1 ? 'Inc' : `${count} inc`,
  decrease: (count: number) => count === 1 ? 'Dec' : `${count} dec`,
  slipStitch: (count: number) => count === 1 ? 'Sl st' : `${count} sl st`,
} as const;

// Error messages
export const ERROR_MESSAGES = {
  FILE_TOO_LARGE: 'File size exceeds maximum allowed size',
  INVALID_FILE_TYPE: 'Invalid file type. Please upload a GLB or GLTF file',
  FILE_READ_ERROR: 'Failed to read file. Please try again',
  MESH_LOAD_ERROR: 'Failed to load 3D model. The file may be corrupted',
  MESH_INVALID: 'Invalid mesh data. Please check your 3D model',
  PATTERN_GENERATION_ERROR: 'Failed to generate pattern. Please try different settings',
  WASM_LOAD_ERROR: 'Failed to load processing module. Please refresh the page',
  NETWORK_ERROR: 'Network error. Please check your connection',
} as const;

// Success messages
export const SUCCESS_MESSAGES = {
  FILE_UPLOADED: 'File uploaded successfully',
  PATTERN_GENERATED: 'Pattern generated successfully',
  PATTERN_EXPORTED: 'Pattern exported successfully',
} as const;

// Local storage keys
export const STORAGE_KEYS = {
  CONFIG: 'crochet_config',
  RECENT_FILES: 'recent_files',
  USER_PREFERENCES: 'user_preferences',
} as const;
