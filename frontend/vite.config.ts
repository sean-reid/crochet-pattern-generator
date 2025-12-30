import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { copyFileSync, mkdirSync } from 'fs'
import { join } from 'path'

export default defineConfig({
  plugins: [
    react(),
    {
      name: 'copy-wasm-files',
      closeBundle() {
        const wasmFiles = ['crochet_wasm_bg.wasm', 'crochet_wasm.js', 'crochet_wasm.d.ts', 'crochet_wasm_bg.wasm.d.ts'];
        try {
          mkdirSync('dist/wasm', { recursive: true });
          wasmFiles.forEach(file => {
            try {
              copyFileSync(
                join('public/wasm', file),
                join('dist/wasm', file)
              );
            } catch (e) {
              // File might not exist, that's ok
            }
          });
          console.log('WASM files copied to dist/wasm');
        } catch (e) {
          console.warn('Could not copy WASM files:', e);
        }
      }
    }
  ],
  server: {
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
  },
  optimizeDeps: {
    exclude: ['three'],
  },
  worker: {
    format: 'es',
    rollupOptions: {
      external: ['/wasm/crochet_wasm.js'],
    },
  },
  publicDir: 'public',
  assetsInclude: ['**/*.wasm'],
  build: {
    rollupOptions: {
      external: ['/wasm/crochet_wasm.js'],
      output: {
        assetFileNames: (assetInfo) => {
          if (assetInfo.name?.endsWith('.wasm')) {
            return 'wasm/[name][extname]';
          }
          return 'assets/[name]-[hash][extname]';
        },
      },
    },
  },
})
