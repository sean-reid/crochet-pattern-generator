import React from 'react';
import { Card } from '../common/Card';
import { Icon } from '../common/Icon';
import styles from './WasmNotLoaded.module.css';

export const WasmNotLoaded: React.FC = () => {
  return (
    <Card className={styles.container}>
      <div className={styles.icon}>
        <Icon name="AlertTriangle" size={48} color="var(--color-terracotta)" />
      </div>
      
      <h2 className={styles.title}>WASM Backend Not Available</h2>
      
      <p className={styles.description}>
        The Rust/WebAssembly backend hasn't been built yet. The frontend is ready and waiting!
      </p>

      <div className={styles.steps}>
        <h3 className={styles.stepsTitle}>To enable full functionality:</h3>
        
        <ol className={styles.stepsList}>
          <li>
            <strong>Navigate to the WASM directory:</strong>
            <code className={styles.code}>cd ../wasm</code>
          </li>
          
          <li>
            <strong>Build the WASM module:</strong>
            <code className={styles.code}>
              wasm-pack build --target web --out-dir ../frontend/public/wasm
            </code>
          </li>
          
          <li>
            <strong>Verify the output files exist:</strong>
            <code className={styles.code}>ls ../frontend/public/wasm/</code>
            <div className={styles.fileList}>
              You should see:
              <ul>
                <li>crochet_pattern_wasm.js</li>
                <li>crochet_pattern_wasm_bg.wasm</li>
                <li>crochet_pattern_wasm.d.ts</li>
              </ul>
            </div>
          </li>
          
          <li>
            <strong>Restart the dev server:</strong>
            <code className={styles.code}>npm run dev</code>
          </li>
        </ol>
      </div>

      <div className={styles.status}>
        <h3 className={styles.statusTitle}>Current Status:</h3>
        <ul className={styles.statusList}>
          <li className={styles.statusReady}>✅ Frontend UI - Ready</li>
          <li className={styles.statusReady}>✅ Web Worker Threading - Ready</li>
          <li className={styles.statusReady}>✅ Type Definitions - Ready</li>
          <li className={styles.statusReady}>✅ Component Integration - Ready</li>
          <li className={styles.statusPending}>⏳ WASM Backend - Needs to be built</li>
        </ul>
      </div>

      <div className={styles.info}>
        <Icon name="Info" size={20} />
        <p>
          Once the WASM module is built, you'll be able to upload 3D models and generate crochet patterns!
        </p>
      </div>
    </Card>
  );
};
