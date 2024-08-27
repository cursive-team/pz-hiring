/* tslint:disable */
/* eslint-disable */
/**
* @param {bigint} input_seed
*/
export function ni_hiring_init_web(input_seed: bigint): void;
/**
* @param {number} id
* @param {number} num_parties
* @returns {any}
*/
export function ni_hiring_client_setup_web(id: number, num_parties: number): any;
/**
* @param {any} sk_share_0
* @param {any} sk_share_1
*/
export function ni_hiring_server_setup_web(sk_share_0: any, sk_share_1: any): void;
/**
* @param {any} ck
* @param {boolean} position
* @param {boolean} commitment
* @param {Uint8Array} combined
* @param {number} salary
* @returns {any}
*/
export function ni_hiring_client_encrypt_web(ck: any, position: boolean, commitment: boolean, combined: Uint8Array, salary: number): any;
/**
* @param {any} jc_fhe_0
* @param {any} jc_fhe_1
* @returns {any}
*/
export function ni_hiring_server_compute_web(jc_fhe_0: any, jc_fhe_1: any): any;
/**
* @param {any} ck
* @param {any} result
* @returns {any}
*/
export function ni_hiring_client_dec_share_web(ck: any, result: any): any;
/**
* @param {any} ck
* @param {any} result
* @param {any} share_0
* @param {any} share_1
* @returns {boolean}
*/
export function ni_hiring_client_full_dec_web(ck: any, result: any, share_0: any, share_1: any): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly ni_hiring_init_web: (a: number) => void;
  readonly ni_hiring_client_setup_web: (a: number, b: number) => number;
  readonly ni_hiring_server_setup_web: (a: number, b: number) => void;
  readonly ni_hiring_client_encrypt_web: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly ni_hiring_server_compute_web: (a: number, b: number) => number;
  readonly ni_hiring_client_dec_share_web: (a: number, b: number) => number;
  readonly ni_hiring_client_full_dec_web: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
