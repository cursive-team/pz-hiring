use phantom_zone::{set_common_reference_seed, set_parameter_set, ParameterSelector};
use rand::{rngs::StdRng, thread_rng, Rng, RngCore, SeedableRng};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::ni_hiring::{
    client_encrypt_job_criteria, client_full_decrypt, client_generate_share, client_setup,
    hiring_match_fhe, server_extract_job_criteria, server_setup, ClientEncryptedData, JobCriteria,
};

#[wasm_bindgen]
pub fn ni_hiring_init_web(input_seed: u64) {
    set_parameter_set(ParameterSelector::NonInteractiveLTE2Party60Bit);
    let mut seed = [0u8; 32];
    let mut rng = StdRng::seed_from_u64(input_seed); // Fixed seed for determinism
    rng.fill_bytes(&mut seed);
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
    position: bool,
    commitment: bool,
    combined: &[u8], // education + experience + interests + company_stage
    salary: u8,
) -> JsValue {
    let criteria_bools: [bool; 20] = combined
        .iter()
        .take(20)
        .map(|&num| num != 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();
    let jc = JobCriteria {
        position,
        commitment,
        education: criteria_bools[0..4].try_into().unwrap(),
        experience: criteria_bools[4..12].try_into().unwrap(),
        interests: criteria_bools[12..16].try_into().unwrap(),
        company_stage: criteria_bools[16..20].try_into().unwrap(),
        salary,
    };
    let ck = serde_wasm_bindgen::from_value(ck).unwrap();
    let s = Serializer::new().serialize_large_number_types_as_bigints(true);
    client_encrypt_job_criteria(jc, ck).serialize(&s).unwrap()
}

#[wasm_bindgen]
pub fn ni_hiring_server_compute_web(jc_fhe_0: JsValue, jc_fhe_1: JsValue) -> JsValue {
    let jc_fhe_0: ClientEncryptedData = serde_wasm_bindgen::from_value(jc_fhe_0).unwrap();
    let jc_fhe_1: ClientEncryptedData = serde_wasm_bindgen::from_value(jc_fhe_1).unwrap();

    let jc_enc_0 = server_extract_job_criteria(0, jc_fhe_0);
    let jc_enc_1 = server_extract_job_criteria(1, jc_fhe_1);

    let res = hiring_match_fhe(jc_enc_0, jc_enc_1);
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
