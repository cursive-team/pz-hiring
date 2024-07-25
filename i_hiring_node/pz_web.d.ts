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
