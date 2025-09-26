use alloy_sol_types::SolType;
use pico_sdk::{client::DefaultProverClient, client::BabyBearProverClient, init_logger};
use std::{borrow::BorrowMut, fs};

pub fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        panic!("Failed to load ELF file from {}: {}", path, err);
    })
}

fn main() {
    // Initialize logger
    init_logger();

    // Load the ELF file
    let elf = load_elf("./prog.elf");

    println!("elf length: {}", elf.len());

    // Initialize the prover client
    let client = BabyBearProverClient::new(&elf);
    let mut stdin_builder = client.new_stdin_builder(); //.get_stdin_builder(); // Shared instance

    // Set up input and generate proof
    let n = 100u32;
    stdin_builder.borrow_mut().write(&n);

    // Generate proof
    let proof = client
        .prove_fast(stdin_builder)
        .expect("Failed to generate proof"); //_fast().expect("Failed to generate proof");

    // Decodes public values from the proof's public value stream.
    let public_buffer = proof.pv_stream.unwrap();
    println!("public_buffer: {:?}", public_buffer);
}
