/// <reference path="../types/wasm.d.ts" />
import * as Comlink from 'comlink';
import type { ProfileCurve, AmigurumiConfig, CrochetPattern } from '../types';

// Type the worker context
declare const self: any;

// We'll store the WASM functions directly
let generate_pattern_from_json: any = null;
let validate_profile: any = null;
let validate_config: any = null;
let init_panic_hook: any = null;
let isInitialized = false;

async function initWasm(): Promise<void> {
  if (isInitialized) return;

  try {
    console.log('Worker initializing WASM...');
    
    // Dev: served from public/wasm/ at /wasm/
    // Prod: served from dist/wasm/ at /wasm/
    // Both resolve to /wasm/crochet_wasm.js from the server root
    const wasmModule = await import(/* @vite-ignore */ '/wasm/crochet_wasm.js');
    
    console.log('WASM module imported, initializing...');
    
    // Initialize the WASM binary
    await wasmModule.default();
    
    console.log('WASM binary loaded');
    
    // Store function references
    generate_pattern_from_json = wasmModule.generate_pattern_from_json;
    validate_profile = wasmModule.validate_profile;
    validate_config = wasmModule.validate_config;
    init_panic_hook = wasmModule.init_panic_hook;
    
    // Initialize panic hook
    if (init_panic_hook) {
      init_panic_hook();
      console.log('Panic hook initialized');
    }
    
    // Verify functions exist
    if (!generate_pattern_from_json) {
      throw new Error('generate_pattern_from_json not found in WASM module');
    }
    
    isInitialized = true;
    console.log('✓ WASM fully initialized');
    
  } catch (error) {
    console.error('WASM initialization failed:', error);
    isInitialized = false;
    throw new Error(`Failed to initialize WASM: ${(error as Error).message}`);
  }
}

export interface PatternGeneratorAPI {
  generatePattern(profile: ProfileCurve, config: AmigurumiConfig): Promise<CrochetPattern>;
  validateProfile(profile: ProfileCurve): Promise<string>;
  validateConfig(config: AmigurumiConfig): Promise<string>;
}

const api: PatternGeneratorAPI = {
  async generatePattern(profile: ProfileCurve, config: AmigurumiConfig): Promise<CrochetPattern> {
    await initWasm();

    if (!generate_pattern_from_json) {
      throw new Error('WASM not initialized');
    }

    try {
      const profileJson = JSON.stringify(profile);
      const configJson = JSON.stringify(config);

      console.log('Calling WASM generate_pattern_from_json...');
      console.log('Profile segments:', profile.segments.length);

      // Call the WASM function directly
      const resultJson = generate_pattern_from_json(profileJson, configJson);
      
      console.log('Pattern generated successfully!');
      
      const pattern: CrochetPattern = JSON.parse(resultJson);
      console.log('Pattern has', pattern.rows.length, 'rows');
      
      return pattern;
    } catch (error) {
      console.error('Pattern generation error:', error);
      throw new Error(`Pattern generation failed: ${(error as Error).message}`);
    }
  },

  async validateProfile(profile: ProfileCurve): Promise<string> {
    await initWasm();

    if (!validate_profile) {
      throw new Error('WASM not initialized');
    }

    try {
      const profileJson = JSON.stringify(profile);
      return validate_profile(profileJson);
    } catch (error) {
      throw new Error(`Validation failed: ${(error as Error).message}`);
    }
  },

  async validateConfig(config: AmigurumiConfig): Promise<string> {
    await initWasm();

    if (!validate_config) {
      throw new Error('WASM not initialized');
    }

    try {
      const configJson = JSON.stringify(config);
      return validate_config(configJson);
    } catch (error) {
      throw new Error(`Validation failed: ${(error as Error).message}`);
    }
  },
};

Comlink.expose(api);
