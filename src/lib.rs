use phantom_zone::{set_common_reference_seed, set_parameter_set, ParameterSelector};
use rand::{thread_rng, RngCore};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

mod ni_hiring;
use ni_hiring::{
    client_encrypt_job_criteria, client_full_decrypt, client_generate_share, client_setup,
    hiring_match_fhe, server_extract_job_criteria, server_setup, ClientEncryptedData, JobCriteria,
    NUM_CRITERIA,
};

#[wasm_bindgen]
pub fn ni_hiring_init_web() {
    set_parameter_set(ParameterSelector::NonInteractiveLTE2Party);
    let mut seed = [0u8; 32];
    thread_rng().fill_bytes(&mut seed);
    set_common_reference_seed(seed);
}

#[wasm_bindgen]
pub fn ni_hiring_client_setup_web(id: u32, num_parties: u32) -> JsValue {
    let s = Serializer::new().serialize_large_number_types_as_bigints(true);
    client_setup(id as usize, num_parties as usize)
        .serialize(&s)
        .unwrap()
}

#[wasm_bindgen]
pub fn ni_hiring_server_setup_web(sk_share_0: JsValue, sk_share_1: JsValue) {
    let server_key_shares = vec![
        serde_wasm_bindgen::from_value(sk_share_0).unwrap(),
        serde_wasm_bindgen::from_value(sk_share_1).unwrap(),
    ];
    server_setup(server_key_shares);
}

#[wasm_bindgen]
pub fn ni_hiring_client_encrypt_web(
    ck: JsValue,
    in_market: bool,
    position: bool,
    salary: u8,
    criteria: &[u8],
) -> JsValue {
    let criteria_bools: [bool; NUM_CRITERIA] = criteria
        .iter()
        .take(NUM_CRITERIA)
        .map(|&num| num != 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();
    let jc = JobCriteria {
        in_market,
        position,
        salary,
        criteria: criteria_bools,
    };
    let ck = serde_wasm_bindgen::from_value(ck).unwrap();
    let s = Serializer::new().serialize_large_number_types_as_bigints(true);
    client_encrypt_job_criteria(jc, ck).serialize(&s).unwrap()
}

#[wasm_bindgen]
pub fn ni_hiring_server_compute_web(jc_fhe_0: JsValue, jc_fhe_1: JsValue) -> JsValue {
    console::log_1(&"Starting deserialization".into());
    let jc_fhe_0: ClientEncryptedData = serde_wasm_bindgen::from_value(jc_fhe_0).unwrap();
    let jc_fhe_1: ClientEncryptedData = serde_wasm_bindgen::from_value(jc_fhe_1).unwrap();

    console::log_1(&"Starting extraction".into());
    let jc_enc_0 = server_extract_job_criteria(0, jc_fhe_0);
    let jc_enc_1 = server_extract_job_criteria(1, jc_fhe_1);

    console::log_1(&"Starting FHE".into());
    let res = hiring_match_fhe(jc_enc_0, jc_enc_1);
    console::log_1(&"Finished FHE".into());

    let s = Serializer::new().serialize_large_number_types_as_bigints(true);
    res.serialize(&s).unwrap()
}

#[wasm_bindgen]
pub fn ni_hiring_client_dec_share_web(ck: JsValue, result: JsValue) -> JsValue {
    let result = serde_wasm_bindgen::from_value(result).unwrap();
    let ck = serde_wasm_bindgen::from_value(ck).unwrap();
    let s = Serializer::new().serialize_large_number_types_as_bigints(true);
    client_generate_share(ck, result).serialize(&s).unwrap()
}

#[wasm_bindgen]
pub fn ni_hiring_client_full_dec_web(
    ck: JsValue,
    result: JsValue,
    share_0: JsValue,
    share_1: JsValue,
) -> bool {
    let result = serde_wasm_bindgen::from_value(result).unwrap();
    let share_0 = serde_wasm_bindgen::from_value(share_0).unwrap();
    let share_1 = serde_wasm_bindgen::from_value(share_1).unwrap();
    let ck = serde_wasm_bindgen::from_value(ck).unwrap();

    client_full_decrypt(ck, result, [share_0, share_1])
}
