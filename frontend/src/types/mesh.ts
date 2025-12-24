export interface Vector3 {
  x: number;
  y: number;
  z: number;
}

export interface Vector2 {
  x: number;
  y: number;
}

export interface BoundingBox {
  min: Vector3;
  max: Vector3;
}

export interface MeshInfo {
  vertexCount: number;
  faceCount: number;
  boundingBox: BoundingBox;
  surfaceArea: number;
  isManifold: boolean;
  hasUVs: boolean;
  hasNormals: boolean;
}

export interface MeshData {
  positions: Float32Array;
  normals?: Float32Array;
  uvs?: Float32Array;
  indices: Uint32Array;
}

export interface ModelFile {
  name: string;
  size: number;
  data: ArrayBuffer;
  meshInfo?: MeshInfo;
}

export interface ValidationError {
  type: 'error' | 'warning';
  message: string;
  details?: string;
}

export interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
  warnings: ValidationError[];
}
