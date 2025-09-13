/* tslint:disable */
/* eslint-disable */
export function start(canvas_id: string): void;
export class WasmGameService {
  free(): void;
  constructor(size: number);
  reset(): void;
  score(): number;
  is_over(): boolean;
  is_won(): boolean;
  slide_left(): boolean;
  slide_right(): boolean;
  slide_up(): boolean;
  slide_down(): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmgameservice_free: (a: number, b: number) => void;
  readonly wasmgameservice_new: (a: number) => number;
  readonly wasmgameservice_reset: (a: number) => void;
  readonly wasmgameservice_score: (a: number) => number;
  readonly wasmgameservice_is_over: (a: number) => number;
  readonly wasmgameservice_is_won: (a: number) => number;
  readonly wasmgameservice_slide_left: (a: number) => number;
  readonly wasmgameservice_slide_right: (a: number) => number;
  readonly wasmgameservice_slide_up: (a: number) => number;
  readonly wasmgameservice_slide_down: (a: number) => number;
  readonly start: (a: number, b: number) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure3_externref_shim: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hec5968579bc0e87b: (a: number, b: number, c: number) => void;
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
