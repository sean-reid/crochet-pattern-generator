import type { ValidationResult, ValidationError } from '../types/mesh';

const SUPPORTED_EXTENSIONS = ['.glb', '.gltf'];
const MAX_FILE_SIZE = 50 * 1024 * 1024; // 50MB
const MIN_FILE_SIZE = 100; // 100 bytes

export const validateFile = (file: File): ValidationResult => {
  const errors: ValidationError[] = [];
  const warnings: ValidationError[] = [];

  // Check file extension
  const extension = file.name.toLowerCase().slice(file.name.lastIndexOf('.'));
  if (!SUPPORTED_EXTENSIONS.includes(extension)) {
    errors.push({
      type: 'error',
      message: 'Unsupported file format',
      details: `Only ${SUPPORTED_EXTENSIONS.join(', ')} files are supported`,
    });
  }

  // Check file size
  if (file.size > MAX_FILE_SIZE) {
    errors.push({
      type: 'error',
      message: 'File too large',
      details: `Maximum file size is ${formatBytes(MAX_FILE_SIZE)}`,
    });
  }

  if (file.size < MIN_FILE_SIZE) {
    errors.push({
      type: 'error',
      message: 'File too small',
      details: 'File appears to be empty or corrupted',
    });
  }

  // Warning for large files
  if (file.size > 10 * 1024 * 1024) {
    warnings.push({
      type: 'warning',
      message: 'Large file detected',
      details: 'Processing may take longer for large files',
    });
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  };
};

export const readFileAsArrayBuffer = (file: File): Promise<ArrayBuffer> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    
    reader.onload = () => {
      if (reader.result instanceof ArrayBuffer) {
        resolve(reader.result);
      } else {
        reject(new Error('Failed to read file as ArrayBuffer'));
      }
    };
    
    reader.onerror = () => {
      reject(new Error('Failed to read file'));
    };
    
    reader.readAsArrayBuffer(file);
  });
};

export const formatBytes = (bytes: number, decimals = 2): string => {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
};

export const getFileExtension = (filename: string): string => {
  return filename.slice(filename.lastIndexOf('.')).toLowerCase();
};

export const isGLBFile = (filename: string): boolean => {
  return getFileExtension(filename) === '.glb';
};

export const isGLTFFile = (filename: string): boolean => {
  return getFileExtension(filename) === '.gltf';
};
