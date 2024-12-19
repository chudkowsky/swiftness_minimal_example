use starknet_crypto::poseidon_hash_many;
use starknet_types_core::felt::Felt;
use swiftness_proof_parser::{parse, StarkProof};
fn main() {
    let proof = std::fs::read_to_string("proof_bridge280135.json").unwrap();
    let parsed_proof = parse(proof).unwrap();
    let program = calculate_program(parsed_proof.clone());
    let program_hash = poseidon_hash_many(&program);
    println!("Program hash: {:?}", program_hash);
}

pub fn calculate_program(proof:StarkProof)->Vec<Felt>{
    let execution_segment = proof
            .public_input
            .segments
            .get(1)
            .unwrap();

        let initial_fp = execution_segment.begin_addr;
        let program_end_pc = initial_fp - 2; // according to https://github.com/HerodotusDev/integrity/blob/main/src/air/layouts/recursive/public_input.cairo#L28-L48
        let program_len = program_end_pc - 1; // Subtract 1 to exclude the last cell, because list starts from 0 and main page is 1-indexed

        let program = &proof.public_input.main_page[0..program_len as usize].to_vec();
        let values = program.iter().map(|cell| cell.value.clone()).collect::<Vec<_>>();
        let mut  felts = vec![];
        for elem in &values {
            felts.push(Felt::from_dec_str(&elem.to_string()).unwrap());
        }
        felts
}


pub fn calculate_output(proof:StarkProof)->Vec<Felt>{
    let output_segment = proof.public_input.segments[2].clone();
    let output_len = output_segment.stop_ptr - output_segment.begin_addr;
    let start = proof.public_input.main_page.len() - output_len as usize;
    let end = proof.public_input.main_page.len();
    let program_output = proof.public_input.main_page[start..end]
            .iter()
            .map(|cell| cell.value.clone()).collect::<Vec<_>>();
    let mut  felts = vec![];
    for elem in &program_output {
        felts.push(Felt::from_dec_str(&elem.to_string()).unwrap());
    }
    felts
}