pub mod condition;
pub mod fulfillment;
pub mod schema;
pub mod util;

use bs58;

use crate::{
    fulfillment::Fulfillment,
    schema::fingerprint::{Ed25519FingerprintContents, Fingerprint},
};

pub trait BaseSha256 {
    fn generate_hash(&self) -> [u8; 64];
}

#[derive(Debug)]
pub struct Ed25519Sha256 {
    pub public_key: Option<[u8; 32]>,
    pub signature: Option<[u8; 64]>,
}

impl Ed25519Sha256 {
    // pub const TYPE_ID: usize = 4;
    // pub const TYPE_NAME: &'static str = "ed25519-sha-256";
    // pub const TYPE_ASN1_CONDITION: &'static str = "ed25519Sha256Condition";
    // pub const TYPE_ASN1_FULFILLMENT: &'static str = "ed25519Sha256Fulfillment";
    // pub const TYPE_CATEGORY: &'static str = "simple";
    // pub const CONSTANT_COST: usize = 131072;

    pub fn new() -> Self {
        Self {
            public_key: None,
            signature: None,
        }
    }

    pub fn set_public_key(&mut self, public_key: [u8; 32]) {
        self.public_key = Some(public_key);
    }
}

impl Fingerprint for Ed25519Sha256 {
    fn get_fingerprint_contents(&self) -> Vec<u8> {
        let buffer = asn1::write_single(&Ed25519FingerprintContents {
            public_key: self.public_key.map(|e| e.to_vec()).as_deref(),
        })
        .expect("write ASN.1 error");

        buffer
    }
}

impl Fulfillment for Ed25519Sha256 {
    const TYPE_ID: usize = 4;
    const TYPE_NAME: &'static str = "ed25519-sha-256";

    fn caculate_cost(&self) -> usize {
        todo!()
    }
}

impl From<&str> for Ed25519Sha256 {
    fn from(public_key: &str) -> Self {
        let public_key = bs58::decode(public_key).into_vec().unwrap();
        let mut buffer = [0u8; 32];
        buffer.clone_from_slice(&public_key[..]);

        let mut ed25519_fulfillment = Self::new();
        ed25519_fulfillment.set_public_key(buffer);

        ed25519_fulfillment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519sha256_fingerprint() {
        let bytes = [1u8; 32];
        let hash = Ed25519Sha256 {
            public_key: Some(bytes),
            signature: None,
        };
        let fingerprint = hash.get_fingerprint_contents();

        assert_eq!(
            fingerprint,
            [
                48, 34, 128, 32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1
            ]
        );
    }
}
