pub mod transaction;

use bs58;
use crypto_conditions::Ed25519Sha256;
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

#[derive(Debug, Serialize)]
pub struct Details {
    #[serde(rename = "type")]
    pub _type: String,
    pub public_key: String,
}

#[derive(Debug, Serialize)]
pub struct JsonBody {
    details: Details,
    uri: String,
}

pub fn cc_jsonify(fulfillment: Ed25519Sha256) -> JsonBody {
    let uri = "fds".to_string();
    let details = Details {
        _type: String::from("ed25519-sha-256"),
        public_key: bs58::encode(fulfillment.public_key.unwrap_or_default()).into_string(),
    };

    JsonBody { details, uri }
}
