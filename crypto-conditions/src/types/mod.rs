mod ed25519_sha256;

pub use ed25519_sha256::*;

pub trait BaseSha256 {
    fn generate_hash(&self) -> [u8; 64];
}
