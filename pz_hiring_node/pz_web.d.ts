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
