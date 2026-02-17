//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use std::time::Instant;

use alloy_sol_types::{sol_data::Array, SolType};
use brotli_decompress::MessageWithMetadataStruct;
use clap::Parser;
use sp1_sdk::{
    blocking::{ProveRequest, Prover, ProverClient},
    include_elf, Elf, ProvingKey, SP1Stdin,
};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
const BROTLI_ELF: Elf = include_elf!("brotli-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(
        long,
        default_value = "000b438086038469752135820422b87b000402f87683064aba80840bebc200840bebc2008401c9c3809407620929b4c0fc48060c4faf378b1e85681c9de1872386f26fc1000080c001a0a47813853bd8456a2f545cffbbd8d6be8292f7e43d6f75cbb61b242ec0a60ab9a0568dd62109b55d483e6407598a69ca62a2e77d25eea4c192962f71eb1ad9c47203"
    )]
    n: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(BROTLI_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        // println!("Output (hex): 0x{}", hex::encode(&output));

        // Read the output.
        let decoded =
            <Array<MessageWithMetadataStruct> as SolType>::abi_decode(output.as_slice()).unwrap();
        // println!("n: {}", args.n);
        println!("decoded: {:?}", decoded[0].message.header.block_number);

        let expected = brotli_decompress::decode_sequencer_message(args.n).unwrap();
        assert_eq!(
            decoded[0].message.header.block_number,
            expected[0].message.header.block_number
        );
        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        let start_time = Instant::now();
        // Setup the program for proving.
        let pk = client.setup(BROTLI_ELF).expect("failed to setup elf");

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        let public_values = proof.public_values.as_slice();
        println!("public values: 0x{}", hex::encode(public_values));

        // Get the proof as bytes.
        let solidity_proof = proof.bytes();
        println!("proof: 0x{}", hex::encode(solidity_proof));

        // Verify the proof.
        client
            .verify(&proof, pk.verifying_key(), None)
            .expect("failed to verify proof");
        println!("Successfully verified proof!");

        proof
            .save("brotli-groth16.bin")
            .expect("saving proof failed");

        println!(
            "Proof generation time: {}ms",
            start_time.elapsed().as_millis()
        );
    }
}
