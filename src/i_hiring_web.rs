use lazy_static::lazy_static;
use phantom_zone::{
    set_common_reference_seed, set_parameter_set, CollectiveKeyShare, FheBool, ParameterSelector,
};
use rand::{thread_rng, Rng, RngCore};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

use crate::i_hiring::{
    client_encrypt_data, client_full_decrypt, client_generate_share, client_setup, server_compute,
    server_setup, ClientEncryptedData, ClientKeys, JobCriteria, NUM_CRITERIA,
};

lazy_static! {
    static ref S: Serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
}

#[wasm_bindgen]
pub fn i_hiring_init() {
    set_parameter_set(ParameterSelector::InteractiveLTE2Party);
    let mut seed = [0u8; 32];
    thread_rng().fill_bytes(&mut seed);
    set_common_reference_seed(seed);
}

#[wasm_bindgen]
pub fn i_hiring_client_setup() -> JsValue {
    client_setup().serialize(&*S).unwrap()
}

#[wasm_bindgen]
pub fn i_hiring_client_encrypt(
    id: u8,
    client_key: JsValue,
    shares: JsValue,
    in_market: bool,
    position: bool,
    salary: u8,
    criteria: &[u8],
) -> JsValue {
    console::log_1(&"1".into());
    let criteria_bools: [bool; NUM_CRITERIA] = criteria
        .iter()
        .take(NUM_CRITERIA)
        .map(|&num| num != 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();
    console::log_1(&"2".into());
    let jc = JobCriteria {
        in_market,
        position,
        salary,
        criteria: criteria_bools,
    };
    console::log_1(&"3".into());
    let ck: ClientKeys = serde_wasm_bindgen::from_value(client_key).unwrap();
    console::log_1(&"4".into());
    let shares: [CollectiveKeyShare; 2] = serde_wasm_bindgen::from_value(shares).unwrap();
    console::log_1(&"5".into());

    client_encrypt_data(id as usize, ck.client_key, &shares, jc)
        .serialize(&*S)
        .unwrap()
}

#[wasm_bindgen]
pub fn i_hiring_server_compute(data_0: JsValue, data_1: JsValue) -> JsValue {
    let data_0: ClientEncryptedData = serde_wasm_bindgen::from_value(data_0).unwrap();
    let data_1: ClientEncryptedData = serde_wasm_bindgen::from_value(data_1).unwrap();
    server_setup(data_0.clone(), data_1.clone());

    server_compute(data_0, data_1).serialize(&*S).unwrap()
}

#[wasm_bindgen]
pub fn i_hiring_client_dec_share(client_key: JsValue, result: JsValue) -> u64 {
    let client_key: ClientKeys = serde_wasm_bindgen::from_value(client_key).unwrap();
    let result: FheBool = serde_wasm_bindgen::from_value(result).unwrap();
    client_generate_share(client_key, result)
}

#[wasm_bindgen]
pub fn i_hiring_client_full_dec(
    client_key: JsValue,
    result: JsValue,
    share_0: u64,
    share_1: u64,
) -> bool {
    let client_key: ClientKeys = serde_wasm_bindgen::from_value(client_key).unwrap();
    let result: FheBool = serde_wasm_bindgen::from_value(result).unwrap();
    client_full_decrypt(client_key, result, [share_0, share_1])
}
