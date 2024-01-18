use hex;
use sha3::{Digest, Sha3_256};

pub fn sha256_hash(data: &str) -> String {
    let mut hash = Sha3_256::new();
    hash.update(data);
    let digest = hash.finalize();
    hex::encode(digest)
}
