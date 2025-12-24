import React, { useState, useRef, useCallback } from 'react';
import { useModelLoader } from '../../hooks/useModelLoader';
import { useApp } from '../../context/AppContext';
import { Button } from '../common/Button';
import { Icon } from '../common/Icon';
import { Card } from '../common/Card';
import { formatBytes } from '../../utils/fileValidation';
import { formatDimensions } from '../../utils/formatters';
import styles from './FileUploadZone.module.css';

const FileUploadZone: React.FC = () => {
  const { modelFile } = useApp();
  const { loadModel, clearModel } = useModelLoader();
  const [isDragging, setIsDragging] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFile = useCallback(async (file: File) => {
    setError(null);
    setIsLoading(true);

    try {
      // Use the WASM-integrated model loader
      const result = await loadModel(file);
      
      if (!result.success) {
        setError(result.error || 'Failed to load file');
      }
    } catch (err) {
      setError('Failed to load file. Please try again.');
      console.error('File loading error:', err);
    } finally {
      setIsLoading(false);
    }
  }, [loadModel]);

  const handleDrop = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);

    const files = Array.from(e.dataTransfer.files);
    if (files.length > 0) {
      handleFile(files[0]);
    }
  }, [handleFile]);

  const handleDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
  }, []);

  const handleClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      handleFile(files[0]);
    }
  };

  const handleRemove = () => {
    clearModel();
    setError(null);
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  if (modelFile && modelFile.meshInfo) {
    return (
      <Card className={styles.fileCard}>
        <div className={styles.fileInfo}>
          <div className={styles.fileHeader}>
            <Icon name="FileText" size={24} color="var(--color-terracotta)" />
            <div className={styles.fileName}>
              <span className={styles.name}>{modelFile.name}</span>
              <span className={styles.size}>{formatBytes(modelFile.size)}</span>
            </div>
          </div>

          <div className={styles.meshInfo}>
            <div className={styles.infoRow}>
              <span className={styles.label}>Vertices:</span>
              <span className={styles.value}>{modelFile.meshInfo.vertexCount.toLocaleString()}</span>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.label}>Faces:</span>
              <span className={styles.value}>{modelFile.meshInfo.faceCount.toLocaleString()}</span>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.label}>Dimensions:</span>
              <span className={styles.value}>{formatDimensions(modelFile.meshInfo.boundingBox)}</span>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.label}>Manifold:</span>
              <span className={styles.value}>{modelFile.meshInfo.isManifold ? 'Yes' : 'No'}</span>
            </div>
          </div>

          <Button variant="secondary" size="small" fullWidth onClick={handleRemove}>
            <Icon name="Trash2" size={16} />
            Remove File
          </Button>
        </div>
      </Card>
    );
  }

  return (
    <div className={styles.uploadZone}>
      <input
        ref={fileInputRef}
        type="file"
        accept=".glb,.gltf"
        onChange={handleFileInput}
        className={styles.fileInput}
        aria-label="Upload 3D model file"
      />

      <div
        className={`${styles.dropZone} ${isDragging ? styles.dragging : ''} ${error ? styles.error : ''}`}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onClick={handleClick}
        role="button"
        tabIndex={0}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            handleClick();
          }
        }}
      >
        {isLoading ? (
          <div className={styles.loading}>
            <div className={styles.spinner} />
            <p className={styles.loadingText}>Loading file...</p>
          </div>
        ) : (
          <>
            <Icon name="Upload" size={48} color="var(--color-terracotta)" />
            <p className={styles.dropText}>
              {isDragging ? 'Drop file here' : 'Drag & drop GLB/GLTF file'}
            </p>
            <p className={styles.orText}>or</p>
            <Button variant="primary" size="medium">
              Browse Files
            </Button>
            <p className={styles.helpText}>Supported formats: .glb, .gltf (Max 50MB)</p>
          </>
        )}
      </div>

      {error && (
        <div className={styles.errorMessage} role="alert">
          <Icon name="AlertCircle" size={16} />
          <span>{error}</span>
        </div>
      )}
    </div>
  );
};

export default FileUploadZone;
