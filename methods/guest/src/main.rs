use json::parse;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256}; // for sha256 hashing
use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub data: u32,
    pub hash: Digest,
}

fn main() {
    // input keys (ACP tree + attributes + MTR)
    // calculate the merkle root of the ACP tree through the attributes
    // assert_eq!(merkle_root, MTR);
    // output with something

    let data: String = env::read();
    let sha = *Impl::hash_bytes(&data.as_bytes());
    let data = parse(&data).unwrap();
    let proven_val = data["uid"].as_u32().unwrap();
    let out = Outputs {
        data: proven_val,
        hash: sha,
    };
    env::commit(&out);
}
