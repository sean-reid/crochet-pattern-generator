import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { copyFileSync, mkdirSync, rmSync } from 'fs'
import { join } from 'path'

export default defineConfig({
  base: './', // Ensures assets are loaded correctly on GitHub Pages sub-paths
  plugins: [
    react(),
    {
      name: 'copy-wasm-files',
      closeBundle() {
        const outDir = join(__dirname, '../docs');
        const wasmFiles = ['crochet_wasm_bg.wasm', 'crochet_wasm.js', 'crochet_wasm.d.ts', 'crochet_wasm_bg.wasm.d.ts'];
        
        try {
          mkdirSync(join(outDir, 'wasm'), { recursive: true });
          wasmFiles.forEach(file => {
            try {
              copyFileSync(
                join(__dirname, 'public/wasm', file),
                join(outDir, 'wasm', file)
              );
            } catch (e) {
              // File might not exist, which is fine
            }
          });
          console.log('WASM files copied to /docs/wasm');
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
      external: ['./wasm/crochet_wasm.js'],
    },
  },
  publicDir: 'public',
  assetsInclude: ['**/*.wasm'],
  build: {
    outDir: '../docs', // Output to the root docs folder
    emptyOutDir: true, // Clear the docs folder before building
    rollupOptions: {
      external: ['./wasm/crochet_wasm.js'],
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
