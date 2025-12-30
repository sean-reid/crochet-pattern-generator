import * as Comlink from 'comlink';
import type { ProfileCurve, AmigurumiConfig, CrochetPattern } from '../types';

// WASM module interface
interface WasmModule {
  init_panic_hook(): void;
  generate_pattern_from_json(profileJson: string, configJson: string): string;
  validate_profile(profileJson: string): string;
  validate_config(configJson: string): string;
}

let wasmModule: WasmModule | null = null;
let isInitialized = false;

async function initWasm(): Promise<void> {
  if (isInitialized) return;

  try {
    // @ts-ignore - WASM imports are dynamic
    const init = (await import('/wasm/crochet_wasm.js')).default;
    await init();

    // @ts-ignore
    wasmModule = await import('/wasm/crochet_wasm.js');
    wasmModule!.init_panic_hook();
    isInitialized = true;
  } catch (error) {
    console.error('Failed to initialize WASM:', error);
    throw new Error('Failed to load WASM module');
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

    if (!wasmModule) {
      throw new Error('WASM module not initialized');
    }

    try {
      const profileJson = JSON.stringify(profile);
      const configJson = JSON.stringify(config);

      const resultJson = wasmModule.generate_pattern_from_json(profileJson, configJson);
      const pattern: CrochetPattern = JSON.parse(resultJson);

      return pattern;
    } catch (error) {
      throw new Error(`Pattern generation failed: ${error}`);
    }
  },

  async validateProfile(profile: ProfileCurve): Promise<string> {
    await initWasm();

    if (!wasmModule) {
      throw new Error('WASM module not initialized');
    }

    try {
      const profileJson = JSON.stringify(profile);
      return wasmModule.validate_profile(profileJson);
    } catch (error) {
      throw new Error(`${error}`);
    }
  },

  async validateConfig(config: AmigurumiConfig): Promise<string> {
    await initWasm();

    if (!wasmModule) {
      throw new Error('WASM module not initialized');
    }

    try {
      const configJson = JSON.stringify(config);
      return wasmModule.validate_config(configJson);
    } catch (error) {
      throw new Error(`${error}`);
    }
  },
};

Comlink.expose(api);
