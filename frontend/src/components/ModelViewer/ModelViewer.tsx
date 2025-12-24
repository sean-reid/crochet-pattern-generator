import React, { useState, useEffect } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Grid, GizmoHelper, GizmoViewport, Center } from '@react-three/drei';
// Use three-stdlib for better TS support in R3F projects
import { GLTFLoader } from 'three-stdlib';
import * as THREE from 'three';
import { useApp } from '../../context/AppContext';
import { Card } from '../common/Card';
import styles from './ModelViewer.module.css';

const ModelViewer: React.FC = () => {
  const { modelFile, pattern } = useApp();
  const [modelScene, setModelScene] = useState<THREE.Group | null>(null);

  useEffect(() => {
    // Only attempt to parse if data is present
    if (!modelFile?.data) return;

    const loader = new GLTFLoader();
    
    // Using .parse to handle the ArrayBuffer already in memory
    loader.parse(
      modelFile.data,
      '',
      (gltf) => {
        setModelScene(gltf.scene);
      },
      (err) => {
        console.error('Error parsing GLB data:', err);
      }
    );

    // Cleanup function to prevent memory leaks
    return () => {
      setModelScene(null);
    };
  }, [modelFile?.data]);

  if (!modelFile) {
    return (
      <Card>
        <div className={styles.placeholder}>
          <div className={styles.placeholderContent}>
            <div className={styles.placeholderIcon}>
              <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
                <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
                <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
                <line x1="12" y1="22.08" x2="12" y2="12" />
              </svg>
            </div>
            <h3 className={styles.placeholderTitle}>No Model Loaded</h3>
            <p className={styles.placeholderText}>
              Upload a 3D model file to begin generating your crochet pattern
            </p>
          </div>
        </div>
      </Card>
    );
  }

  return (
    <div className={styles.viewerContainer}>
      <Card>
        <div className={styles.viewer}>
          <Canvas
            camera={{ position: [5, 5, 5], fov: 50 }}
            className={styles.canvas}
          >
            <color attach="background" args={['#2A2A2A']} />
            
            <ambientLight intensity={0.7} />
            <pointLight position={[10, 10, 10]} intensity={1.5} />
            <spotLight position={[-10, 20, 10]} angle={0.15} penumbra={1} intensity={2} />

            <Center>
              {/* Display the parsed WASM model */}
              {modelScene && <primitive object={modelScene} dispose={null} />}

              {/* Display stitches from the pattern */}
              {pattern?.stitches && pattern.stitches.map((stitch) => (
                <mesh
                  key={stitch.id}
                  position={[stitch.position3D.x, stitch.position3D.y, stitch.position3D.z]}
                >
                  <sphereGeometry args={[0.03, 12, 12]} />
                  <meshStandardMaterial 
                    color={stitch.type === 'inc' ? '#E89B87' : stitch.type === 'dec' ? '#8BA888' : '#C67B5C'}
                    emissive={stitch.type === 'inc' ? '#E89B87' : stitch.type === 'dec' ? '#8BA888' : '#C67B5C'}
                    emissiveIntensity={0.1}
                  />
                </mesh>
              ))}
            </Center>

            <Grid
              args={[10, 10]}
              cellSize={1}
              cellThickness={0.5}
              cellColor="#E8E8E8"
              sectionSize={5}
              sectionThickness={1}
              sectionColor="#6B6B6B"
              fadeDistance={30}
              fadeStrength={1}
              position={[0, -1, 0]}
            />

            <OrbitControls
              makeDefault
              enableDamping
              dampingFactor={0.05}
              minDistance={2}
              maxDistance={20}
            />

            <GizmoHelper alignment="bottom-right" margin={[80, 80]}>
              <GizmoViewport
                axisColors={['#ff0000', '#00ff00', '#0000ff']}
                labelColor="white"
              />
            </GizmoHelper>
          </Canvas>

          <div className={styles.controls}>
            <div className={styles.modelStats}>
              {/* Using the meshInfo properties we mapped in useModelLoader */}
              <span>{modelFile.meshInfo?.vertexCount.toLocaleString()} Vertices</span>
              <span>{modelFile.meshInfo?.faceCount.toLocaleString()} Faces</span>
            </div>
            <div className={styles.hint}>
              <span>Left click + drag to rotate</span>
              <span>Right click + drag to pan</span>
              <span>Scroll to zoom</span>
            </div>
          </div>
        </div>
      </Card>
    </div>
  );
};

export default ModelViewer;
