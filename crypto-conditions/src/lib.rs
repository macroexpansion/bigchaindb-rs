pub mod condition;
pub mod fulfillment;
pub mod schema;
pub mod util;

use bs58;
use ring_compat::signature::{ed25519::SigningKey, Signer};

use crate::{
    fulfillment::Fulfillment,
    schema::fingerprint::{Ed25519FingerprintContents, Fingerprint},
};

pub trait BaseSha256 {
    fn generate_hash(&self) -> [u8; 64];
}

pub struct Ed25519Signer<S>
where
    S: Signer<ed25519::Signature>,
{
    pub signing_key: S,
}

impl<S> Ed25519Signer<S>
where
    S: Signer<ed25519::Signature>,
{
    pub fn sign(&self, message: &[u8]) -> ed25519::Signature {
        // NOTE: use `try_sign` if you'd like to be able to handle
        // errors from external signing services/devices (e.g. HSM/KMS)
        // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
        self.signing_key.sign(message)
    }
}

pub type RingEd25519Signer = Ed25519Signer<SigningKey>;

#[derive(Debug)]
pub struct Ed25519Sha256 {
    pub public_key: Option<[u8; 32]>,
    pub signature: Option<[u8; 64]>,
}

impl Ed25519Sha256 {
    // pub const TYPE_ASN1_CONDITION: &'static str = "ed25519Sha256Condition";
    // pub const TYPE_ASN1_FULFILLMENT: &'static str = "ed25519Sha256Fulfillment";
    // pub const TYPE_CATEGORY: &'static str = "simple";

    pub fn new() -> Self {
        Self {
            public_key: None,
            signature: None,
        }
    }

    pub fn set_public_key(&mut self, public_key: [u8; 32]) {
        self.public_key = Some(public_key);
    }

    pub fn sign(&mut self, message: &[u8], private_key: &[u8; 32]) {
        let signing_key = SigningKey::from_bytes(&private_key);
        let verifying_key = signing_key.verifying_key();
        self.public_key = Some(verifying_key.0);

        let signer = RingEd25519Signer { signing_key };
        let signature = signer.sign(message);
        self.signature = Some(signature.to_bytes());
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
    const CONSTANT_COST: usize = 131072;
}

impl From<&str> for Ed25519Sha256 {
    fn from(public_key: &str) -> Self {
        let public_key = bs58::decode(public_key).into_vec().unwrap();
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&public_key[..]);

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

    #[test]
    fn test_ed25519sha256_fulfillment() {
        let mut hash = Ed25519Sha256::new();
        hash.set_public_key([1u8; 32]);

        println!("{:?}", hash.generate_hash());

        assert_eq!(hash.get_type_id(), 4);
        assert_eq!(hash.get_type_name(), "ed25519-sha-256");
        assert_eq!(hash.caculate_cost(), 131072);
        assert_eq!(
            hash.generate_hash(),
            [
                73, 36, 153, 193, 199, 220, 115, 190, 177, 28, 106, 24, 227, 194, 108, 81, 56, 106,
                209, 202, 250, 126, 2, 86, 9, 31, 37, 95, 31, 94, 228, 93
            ]
        );
        assert_eq!(hash.get_condition_uri(), "ni:///sha-256;SSSZwcfcc76xHGoY48JsUThq0cr6fgJWCR8lXx9e5F0?fpt=ed25519-sha-256&cost=131072");
    }

    #[test]
    fn test_ed25519_sign() {
        let pubkey = "6zaQbbRi7RCFhCF35tpVDu2nEfR9fZCqx2MvUa7pKRmX";
        let prikey = "CHwxsNPzRXTzCz25KZ9TJcBJ45H25JKkLL4HrX1nBfXT";

        let private_key = bs58::decode(prikey).into_vec().unwrap();
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&private_key[..]);

        let mut hash = Ed25519Sha256::new();
        let message = "Hello, world";
        hash.sign(message.as_bytes(), &buffer);

        assert_eq!(
            bs58::encode(hash.signature.unwrap()).into_string(),
            "5DTN5U1C3rEsVKADyMkqVEzKQ6kVbkCtuCWf28iuqJnaeDtFmLAamwfqFV6LMwBNkJM9iU1UkXRmdwBUdYAc5yTU"
        );
        assert_eq!(bs58::encode(hash.public_key.unwrap()).into_string(), pubkey);
    }
}
