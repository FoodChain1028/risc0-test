// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID
};
use risc0_zkvm::{default_prover,  ExecutorEnv};
use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub data: u32,
    pub hash: Digest,
}


fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().
    let data = include_str!("../../example.json");
    println!("Data: {:?}", data);
    let out = search_json(data);
    println!("Output: {:?}", out);

    println!();
    println!("  {:?}", out.hash);
    println!(
        "provably contains a field 'uid' with value {}",
        out.data
    );
}

fn search_json(data: &str) -> Outputs {
    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF).unwrap().receipt;

    // example of how someone else could verify this receipt.
    // we can use verifier contract to verify this proof + imageId(like the unique id for our app).
    receipt
        .verify(GUEST_CODE_FOR_ZK_PROOF_ID)
        .unwrap();

    receipt.journal.decode().unwrap()
}
