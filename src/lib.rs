pub mod error;
pub mod request;
pub mod sha256_hash;
pub mod transaction;
pub mod transport;

use bs58;
use crypto_conditions::{fulfillment::Fulfillment, Ed25519Sha256};
use rand::RngCore;
use serde::Serialize;
use tweetnacl;

pub fn randombytes(seed: &mut [u8; 32]) {
    let mut rng = rand::thread_rng();
    rng.fill_bytes(seed);
}

#[derive(Debug)]
pub struct Ed25519Keypair {
    pub pk: String,
    pub sk: String,
}

pub fn ed25519_keypair() -> Ed25519Keypair {
    let mut pk: [u8; 32] = [0; 32];
    let mut sk: [u8; 64] = [0; 64];
    let mut seed: [u8; 32] = [0; 32];

    randombytes(&mut seed);
    tweetnacl::sign_keypair_seed(&mut pk, &mut sk, &seed);

    let sk = &sk[0..32];
    let pk = bs58::encode(pk).into_string();
    let sk = bs58::encode(sk).into_string();
    Ed25519Keypair { pk, sk }
}

/// Fields of this struct needed to be sorted alphabetically
#[derive(Debug, Serialize, Clone)]
pub struct Details {
    pub public_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

/// Fields of this struct needed to be sorted alphabetically
#[derive(Debug, Serialize, Clone)]
pub struct JsonBody {
    details: Details,
    uri: String,
}

pub fn cc_jsonify(fulfillment: Ed25519Sha256) -> Option<JsonBody> {
    let condition_uri = fulfillment.get_condition_uri();

    if fulfillment.get_type_id() == Ed25519Sha256::TYPE_ID {
        let details = Details {
            type_: String::from(Ed25519Sha256::TYPE_NAME),
            public_key: bs58::encode(fulfillment.public_key.unwrap_or_default()).into_string(),
        };

        return Some(JsonBody {
            details,
            uri: condition_uri,
        });
    }

    None
}
