use hex::ToHex;
use sha3::{Digest, Keccak256, Sha3_256};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Expected 2 input (<type>, <input>), received {:?}",
            args.len() - 1
        );
        return;
    }

    if args[1].to_lowercase().eq("keccak256") {
        let mut hasher = Keccak256::new();
        hasher.update(args[2].as_bytes());

        let result = hasher.finalize();
        println!("{:?} -> {:?}", args[2], result.encode_hex::<String>());
    } else if args[1].to_lowercase().eq("sha3_256") {
        let mut hasher = Sha3_256::new();
        hasher.update(args[2].as_bytes());

        let result = hasher.finalize();
        println!("{:?} -> {:?}", args[2], result.encode_hex::<String>());
    } else {
        println!(
            "Expected hash type to be one of <keccack256 | sha3_256>, received {:?}",
            args[1]
        );
    }
}
