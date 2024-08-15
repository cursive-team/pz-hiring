use phantom_zone::*;
use serde::{Deserialize, Serialize};

/**
 * HIRING MP-FHE MATCHING SPEC
 * - match two people in job market (recruiters, job hunters)
 * - match hunter with recruiter
 * - match hunter salary request < recruiter salary provided
 * - hunter fits all requirements of recruiter
 */

pub const NUM_CRITERIA: usize = 3;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobCriteria {
    pub position: bool, // 0 = hunter, 1 = recruiter
    pub commitment: bool,
    pub education: [bool; 4],
    pub experience: [bool; 8],
    pub interests: [bool; 4],
    pub company_stage: [bool; 4],
    pub salary: u8, // x * $3,000
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FheJobCriteria {
    pub position: FheBool,
    pub commitment: FheBool,
    pub education: [FheBool; 4],
    pub experience: [FheBool; 8],
    pub interests: [FheBool; 4],
    pub company_stage: [FheBool; 4],
    pub salary: FheUint8,
}

fn hiring_match(a: JobCriteria, b: JobCriteria) -> bool {
    // check a is a recruiter and b is searcher
    let compatible_pos = a.position ^ b.position;
    let a_recruiter = a.position;

    // check qualifications
    let mut education_match = a.education[0] & b.education[0];
    for i in 1..4 {
        education_match |= a.education[i] & b.education[i];
    }
    let mut experience_match = a.experience[0] & b.experience[0];
    for i in 1..8 {
        experience_match |= a.experience[i] & b.experience[i];
    }

    // check overlap in opportunity
    let salary_match = a.salary > b.salary;
    let mut interest_overlap = a.interests[0] & b.interests[0];
    for i in 1..4 {
        interest_overlap |= a.interests[i] & b.interests[i];
    }
    let mut stage_overlap = a.company_stage[0] & b.company_stage[0];
    for i in 1..4 {
        stage_overlap |= a.company_stage[i] & b.company_stage[i];
    }

    // check alignment on commitment
    let commitment_overlap = !a.commitment | b.commitment;

    compatible_pos
        & a_recruiter
        & education_match
        & experience_match
        & salary_match
        & interest_overlap
        & stage_overlap
        & commitment_overlap
}

pub fn hiring_match_fhe(a: FheJobCriteria, b: FheJobCriteria) -> FheBool {
    // check a is a recruiter and b is searcher
    let compatible_pos = &a.position ^ &b.position;
    let a_recruiter = &a.position;

    // check qualifications
    let mut education_match = &a.education[0] & &b.education[0];
    for i in 1..4 {
        education_match |= &a.education[i] & &b.education[i];
    }
    let mut experience_match = &a.experience[0] & &b.experience[0];
    for i in 1..8 {
        experience_match |= &a.experience[i] & &b.experience[i];
    }

    // check overlap in opportunity
    let salary_match = &a.salary.gt(&b.salary);
    let mut interest_overlap = &a.interests[0] & &b.interests[0];
    for i in 1..4 {
        interest_overlap |= &a.interests[i] & &b.interests[i];
    }
    let mut stage_overlap = &a.company_stage[0] & &b.company_stage[0];
    for i in 1..4 {
        stage_overlap |= &a.company_stage[i] & &b.company_stage[i];
    }

    // check alignment on commitment
    let commitment_overlap = &!&a.commitment | &b.commitment;

    &(&(&(&(&(&(&compatible_pos & &a_recruiter) & &education_match) & &experience_match)
        & salary_match)
        & &interest_overlap)
        & &stage_overlap)
        & &commitment_overlap
}

/**
 * FHE SETUP CODE
 */

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientKeys {
    client_key: ClientKey,
    server_key_share: ServerKeyShare,
}

pub fn client_setup(id: usize, num_parties: usize) -> ClientKeys {
    let client_key = gen_client_key();
    let server_key_share = gen_server_key_share(id, num_parties, &client_key); // Changed `ck` to `client_key`

    ClientKeys {
        client_key,
        server_key_share,
    }
}

pub fn server_setup(server_key_shares: Vec<ServerKeyShare>) {
    let server_key = aggregate_server_key_shares(&server_key_shares);
    server_key.set_server_key();
}

/**
 * FHE FUNCTION EVAL CODE
 */

#[derive(Serialize, Deserialize)]
pub struct ClientEncryptedData {
    bool_enc: NonInteractiveSeededFheBools<Vec<u64>, [u8; 32]>,
    salary_enc: EncFheUint8,
}

pub fn client_encrypt_job_criteria(jc: JobCriteria, ck: ClientKey) -> ClientEncryptedData {
    let bool_vec = ([jc.position, jc.commitment].iter().copied())
        .chain(jc.education.iter().copied())
        .chain(jc.experience.iter().copied())
        .chain(jc.interests.iter().copied())
        .chain(jc.company_stage.iter().copied())
        .collect::<Vec<_>>();

    let bool_enc = ck.encrypt(bool_vec.as_slice());
    let salary_enc = ck.encrypt(vec![jc.salary].as_slice());

    ClientEncryptedData {
        bool_enc,
        salary_enc,
    }
}

pub fn server_extract_job_criteria(id: usize, data: ClientEncryptedData) -> FheJobCriteria {
    let tmp = data
        .bool_enc
        .unseed::<Vec<Vec<u64>>>()
        .key_switch(id)
        .extract_all();
    let (position, commitment) = { (tmp[0].clone(), tmp[1].clone()) };

    let mut education: [FheBool; 4] = Default::default();
    for i in 0..4 {
        education[i] = tmp[2 + i].clone();
    }

    let mut experience: [FheBool; 8] = Default::default();
    for i in 0..8 {
        experience[i] = tmp[6 + i].clone();
    }

    let mut interests: [FheBool; 4] = Default::default();
    for i in 0..4 {
        interests[i] = tmp[14 + i].clone();
    }

    let mut company_stage: [FheBool; 4] = Default::default();
    for i in 0..4 {
        company_stage[i] = tmp[18 + i].clone();
    }

    let salary = data
        .salary_enc
        .unseed::<Vec<Vec<u64>>>()
        .key_switch(id)
        .extract_at(0);

    FheJobCriteria {
        position,
        commitment,
        education,
        experience,
        interests,
        company_stage,
        salary,
    }
}

/**
 * FHE DECRYPTION CODE
 */

pub fn client_generate_share(ck: ClientKeys, result: FheBool) -> u64 {
    ck.client_key.gen_decryption_share(&result)
}

pub fn client_full_decrypt(ck: ClientKeys, result: FheBool, shares: [u64; 2]) -> bool {
    ck.client_key.aggregate_decryption_shares(&result, &shares)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, RngCore};

    #[test]
    fn ni_hiring_query() {
        set_parameter_set(ParameterSelector::NonInteractiveLTE2Party60Bit);

        /*
         * Phase 1: KEY SETUP
         */
        println!("Noninteractive MP-FHE Key Setup");

        // set application's common reference seed
        let mut seed = [0u8; 32];
        thread_rng().fill_bytes(&mut seed);
        set_common_reference_seed(seed);

        // Client setup
        let mut now = std::time::Instant::now();
        let ck_0 = client_setup(0, 2);
        let ck_1 = client_setup(1, 2);
        println!(
            "(1) Client keys + server shares generated, {:?}ms",
            now.elapsed().as_millis()
        );

        // Server setup
        now = std::time::Instant::now();
        server_setup(vec![
            ck_0.clone().server_key_share,
            ck_1.clone().server_key_share,
        ]);
        println!(
            "(2) Server key aggregated, {:?}ms",
            now.elapsed().as_millis()
        );

        /*
         * Phase 2: FUNCTION COMPUTATION
         */

        println!("\nFunction computation");

        // Client encryption
        now = std::time::Instant::now();

        let jc_0 = JobCriteria {
            position: true, // recruiter
            commitment: true,
            education: [false, false, true, true],
            experience: [false, false, true, true, true, true, true, true],
            interests: [false, true, true, false],
            company_stage: [false, true, false, false],
            salary: 200, // asking for at least 1mil
        };
        let jc_1 = JobCriteria {
            position: false, // searcher
            commitment: true,
            education: [true, true, true, false],
            experience: [true, true, true, true, false, false, false, false],
            interests: [true, false, true, false],
            company_stage: [true, true, false, false],
            salary: 150, // asking for at least 1mil
        };
        let data_0 = client_encrypt_job_criteria(jc_0.clone(), ck_0.client_key.clone());
        let data_1 = client_encrypt_job_criteria(jc_1.clone(), ck_1.client_key.clone());
        println!(
            "(1) Clients encrypt their input with their own key, {:?}ms",
            now.elapsed().as_millis()
        );

        // Server extracting data from ciphertext
        now = std::time::Instant::now();
        let jc_fhe_0 = server_extract_job_criteria(0, data_0);
        let jc_fhe_1 = server_extract_job_criteria(1, data_1);
        println!(
            "(2) Client inputs extracted after key switch, {:?}ms",
            now.elapsed().as_millis()
        );

        // Server evaluating function
        now = std::time::Instant::now();
        let match_res = hiring_match(jc_0.clone(), jc_1.clone());
        let match_res_fhe = hiring_match_fhe(jc_fhe_0, jc_fhe_1);
        println!("(3) f1 evaluated, {:?}ms", now.elapsed().as_millis());

        // Clients produce decryption share
        now = std::time::Instant::now();
        let decryption_shares = [
            client_generate_share(ck_0.clone(), match_res_fhe.clone()),
            client_generate_share(ck_1, match_res_fhe.clone()),
        ];
        println!(
            "(4) Decryption shares generated, {:?}ms",
            now.elapsed().as_millis()
        );

        // Clients aggregate shares to decrypt
        now = std::time::Instant::now();
        let out_f1 = client_full_decrypt(ck_0, match_res_fhe, decryption_shares);
        println!(
            "(5) Decryption shares aggregated, data decrypted by client, {:?}ms",
            now.elapsed().as_millis()
        );

        println!("\nResult comparison");
        println!("Plaintext result: {}", match_res);
        println!("FHE result: {}", out_f1);
    }
}
