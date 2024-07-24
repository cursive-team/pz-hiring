/* tslint:disable */
/* eslint-disable */
/**
* @param {bigint} input_seed
*/
export function i_hiring_init(input_seed: bigint): void;
/**
* @returns {any}
*/
export function i_hiring_client_setup(): any;
/**
* @param {number} id
* @param {any} client_key
* @param {any} shares
* @param {boolean} in_market
* @param {boolean} position
* @param {number} salary
* @param {Uint8Array} criteria
* @returns {any}
*/
export function i_hiring_client_encrypt(id: number, client_key: any, shares: any, in_market: boolean, position: boolean, salary: number, criteria: Uint8Array): any;
/**
* @param {any} data_0
* @param {any} data_1
* @returns {any}
*/
export function i_hiring_server_compute(data_0: any, data_1: any): any;
/**
* @param {any} client_key
* @param {any} result
* @returns {bigint}
*/
export function i_hiring_client_dec_share(client_key: any, result: any): bigint;
/**
* @param {any} client_key
* @param {any} result
* @param {bigint} share_0
* @param {bigint} share_1
* @returns {boolean}
*/
export function i_hiring_client_full_dec(client_key: any, result: any, share_0: bigint, share_1: bigint): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly i_hiring_init: (a: number) => void;
  readonly i_hiring_client_setup: () => number;
  readonly i_hiring_client_encrypt: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly i_hiring_server_compute: (a: number, b: number) => number;
  readonly i_hiring_client_dec_share: (a: number, b: number) => number;
  readonly i_hiring_client_full_dec: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
