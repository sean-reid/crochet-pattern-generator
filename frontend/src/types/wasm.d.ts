declare module '/wasm/crochet_wasm.js' {
  export default function init(): Promise<void>;
  export function init_panic_hook(): void;
  export function generate_pattern_from_json(
    profile_json: string,
    config_json: string
  ): string;
  export function validate_profile(profile_json: string): string;
  export function validate_config(config_json: string): string;
}
