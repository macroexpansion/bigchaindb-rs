pub mod condition;
pub mod fulfillment;
pub mod util;

use bs58;

use crate::fulfillment::Fulfillment;

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

impl Fulfillment for Ed25519Sha256 {
    const TYPE_ID: usize = 4;
    const TYPE_NAME: &'static str = "ed25519-sha-256";

    fn generate_hash(&self) -> [u8; 32] {
        todo!()
    }

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
