/* tslint:disable */
/* eslint-disable */
/**
 * @returns {any}
 */
export function generate_lotto_numbers(): any;
/**
 * @returns {any}
 */
export function generate_lotto_numbers_wasm(): any;
/**
 * @returns {any}
 */
export function generate_pension_numbers(): any;
/**
 * @returns {any}
 */
export function generate_pension_numbers_wasm(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly generate_lotto_numbers: () => number;
  readonly generate_pension_numbers: () => number;
  readonly generate_lotto_numbers_wasm: () => number;
  readonly generate_pension_numbers_wasm: () => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
