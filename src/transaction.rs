#![allow(dead_code)]

use std::collections::HashSet;

use crypto_conditions::{self, fulfillment::Fulfillment, Ed25519Sha256};
use serde::Serialize;

use crate::{cc_jsonify, sha256_hash::sha256_hash, Details, JsonBody};

#[derive(Debug, Serialize, Clone)]
pub enum Operation {
    CREATE,
    TRANSFER,
}

#[derive(Debug, Serialize, Clone)]
pub struct AssetDefinition<T: Serialize + Clone> {
    pub data: Option<T>,
}

/// Fields of this struct needed to be sorted alphabetically
#[derive(Debug, Serialize, Clone)]
pub struct InputTemplate {
    pub fulfillment: Option<String>,
    pub fulfills: Option<String>,
    pub owners_before: Vec<String>,
}

impl InputTemplate {
    pub fn new(
        public_keys: Vec<String>,
        fulfills: Option<String>,
        fulfillment: Option<String>,
    ) -> Self {
        Self {
            fulfillment,
            fulfills,
            owners_before: public_keys,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Ed25519Condition {
    detail: String,
}

/// Fields of this struct needed to be sorted alphabetically
#[derive(Debug, Serialize, Clone)]
pub struct Output {
    pub amount: String,
    pub condition: JsonBody,
    pub public_keys: Vec<String>,
}

/// Fields of this struct needed to be sorted alphabetically
#[derive(Debug, Serialize, Clone)]
pub struct TransactionTemplate<M: Serialize + Clone, A: Serialize + Clone> {
    pub asset: Option<A>,
    pub id: Option<String>,
    pub inputs: Vec<InputTemplate>,
    pub metadata: Option<M>,
    pub operation: Option<Operation>,
    pub outputs: Vec<Output>,
    pub version: String,
}

impl<M: Serialize + Clone, A: Serialize + Clone> TransactionTemplate<M, A> {
    pub fn new() -> Self {
        Self {
            id: None,
            operation: None,
            outputs: Vec::new(),
            inputs: Vec::new(),
            metadata: None,
            asset: None,
            version: String::from("2.0"),
        }
    }

    pub fn serialize_transaction_into_canonical_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub struct Transaction;

impl Transaction {
    fn make_transaction(
        operation: Operation,
        asset: impl Serialize + Clone,
        metadata: impl Serialize + Clone,
        outputs: Vec<Output>,
        inputs: Vec<InputTemplate>,
    ) -> TransactionTemplate<impl Serialize + Clone, impl Serialize + Clone> {
        let mut tx = TransactionTemplate::new();
        tx.operation = Some(operation);
        tx.asset = Some(asset);
        tx.metadata = Some(metadata);
        tx.inputs = inputs;
        tx.outputs = outputs;
        tx
    }

    /// Generate a `CREATE` transaction holding the `asset`, `metadata`, and `outputs`, to be signed by the `issuers`.
    pub fn make_create_transaction(
        asset: Option<impl Serialize + Clone>,
        metadata: impl Serialize + Clone,
        outputs: Vec<Output>,
        issuers: Vec<String>,
    ) -> TransactionTemplate<impl Serialize + Clone, impl Serialize + Clone> {
        let asset_definition = AssetDefinition { data: asset };
        let inputs: Vec<InputTemplate> = issuers
            .iter()
            .map(|issuer| InputTemplate::new(vec![issuer.to_string()], None, None))
            .collect();

        Self::make_transaction(
            Operation::CREATE,
            asset_definition,
            metadata,
            outputs,
            inputs,
        )
    }

    fn make_ed25519_condition(pubkey: &str, json: bool) -> Option<JsonBody> {
        let fulfillment = Ed25519Sha256::from(pubkey);

        // TODO: implement this from js code
        // return json ? ccJsonify(ed25519Fulfillment) : ed25519Fulfillment
        if json {
            cc_jsonify(fulfillment)
        } else {
            todo!()
        }
    }

    fn make_output(condition: JsonBody, amount: String) -> Output {
        let mut public_keys = HashSet::new();
        let mut get_public_keys = |details: &Details| {
            if details.type_ == Ed25519Sha256::TYPE_NAME {
                public_keys.insert(details.public_key.clone());
            } else {
                // TODO: implement this from js code
                // } else if (details.type === 'threshold-sha-256') {
                //     details.subconditions.map(getPublicKeys)
                // }
                todo!()
            }
        };

        get_public_keys(&condition.details);

        Output {
            condition,
            amount,
            public_keys: Vec::from_iter(public_keys),
        }
    }

    fn sign_transaction<A, M>(
        transaction: &TransactionTemplate<A, M>,
        private_keys: Vec<&str>,
    ) -> TransactionTemplate<A, M>
    where
        A: Serialize + Clone,
        M: Serialize + Clone,
    {
        let mut signed_transaction: TransactionTemplate<A, M> = transaction.clone();
        let serialized_transaction = transaction.serialize_transaction_into_canonical_string();

        for (index, input_template) in signed_transaction.inputs.iter_mut().enumerate() {
            let private_key = private_keys[index];
            let private_key = bs58::decode(private_key).into_vec().unwrap();

            let transaction_unique_fulfillment: &str =
                if let Some(_fulfills) = &input_template.fulfills {
                    // TODO: implement this from js code
                    // serializedTransaction
                    //     .concat(input.fulfills.transaction_id)
                    //     .concat(input.fulfills.output_index)
                    todo!()
                } else {
                    &serialized_transaction
                };

            let transaction_hash = sha256_hash(transaction_unique_fulfillment);

            let mut ed25519_fulfillment = Ed25519Sha256::new();
            let mut buffer = [0u8; 32];
            buffer.copy_from_slice(&private_key[..]);
            let transaction_hash = hex::decode(transaction_hash).unwrap();
            ed25519_fulfillment.sign(&transaction_hash, &buffer);

            let fulfillment_uri = ed25519_fulfillment.serialize_uri();

            input_template.fulfillment = Some(fulfillment_uri);
        }

        let serialized_signed_transaction =
            signed_transaction.serialize_transaction_into_canonical_string();
        signed_transaction.id = Some(sha256_hash(&serialized_signed_transaction));

        signed_transaction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_ed25519_condition_with_cc_jsonify() {
        let bytes = [1u8; 32];
        let pk = bs58::encode(bytes).into_string();

        let condition = Transaction::make_ed25519_condition(&pk, true).unwrap();

        assert_eq!(condition.details.type_, "ed25519-sha-256");
        assert_eq!(
            condition.details.public_key,
            "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"
        );
        assert_eq!(condition.uri, "ni:///sha-256;SSSZwcfcc76xHGoY48JsUThq0cr6fgJWCR8lXx9e5F0?fpt=ed25519-sha-256&cost=131072");
    }

    #[test]
    fn test_make_output() {
        let bytes = [1u8; 32];
        let pk = bs58::encode(bytes).into_string();

        let condition = Transaction::make_ed25519_condition(&pk, true).unwrap();

        assert_eq!(condition.details.type_, "ed25519-sha-256");
        assert_eq!(
            condition.details.public_key,
            "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"
        );
        assert_eq!(condition.uri, "ni:///sha-256;SSSZwcfcc76xHGoY48JsUThq0cr6fgJWCR8lXx9e5F0?fpt=ed25519-sha-256&cost=131072");

        let output = Transaction::make_output(condition, String::from("1"));

        assert_eq!(&output.amount, "1");
        assert_eq!(
            output.public_keys.first().unwrap(),
            "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"
        );
    }

    #[test]
    fn test_make_create_transaction_and_stable_stringify() {
        let assetdata = serde_json::json!({
            "ft": {
                "signature": "signature",
                "device": "device",
            }
        });
        let metadata = serde_json::json!({"metadata": "metadata"});
        let asset = Some(assetdata);

        let bytes = [1u8; 32];
        let pk = bs58::encode(bytes).into_string();
        let condition = Transaction::make_ed25519_condition(&pk, true).unwrap();

        let output = Transaction::make_output(condition, String::from("1"));

        let transaction =
            Transaction::make_create_transaction(asset, metadata, vec![output], vec![pk]);
        let json = transaction.serialize_transaction_into_canonical_string();

        let json_target = r#"{"asset":{"data":{"ft":{"device":"device","signature":"signature"}}},"id":null,"inputs":[{"fulfillment":null,"fulfills":null,"owners_before":["4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"]}],"metadata":{"metadata":"metadata"},"operation":"CREATE","outputs":[{"amount":"1","condition":{"details":{"public_key":"4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi","type":"ed25519-sha-256"},"uri":"ni:///sha-256;SSSZwcfcc76xHGoY48JsUThq0cr6fgJWCR8lXx9e5F0?fpt=ed25519-sha-256&cost=131072"},"public_keys":["4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"]}],"version":"2.0"}"#;
        assert_eq!(json, json_target);
    }

    #[test]
    fn test_sign_transaction() {
        let assetdata = serde_json::json!({
            "ft": {
                "signature": "signature",
                "device": "device",
            }
        });
        let metadata = serde_json::json!({"metadata": "metadata"});
        let asset = Some(assetdata);

        let public_key = "6zaQbbRi7RCFhCF35tpVDu2nEfR9fZCqx2MvUa7pKRmX";
        let private_key = "CHwxsNPzRXTzCz25KZ9TJcBJ45H25JKkLL4HrX1nBfXT";
        let condition = Transaction::make_ed25519_condition(&public_key, true).unwrap();

        let output = Transaction::make_output(condition, String::from("1"));

        let transaction = Transaction::make_create_transaction(
            asset,
            metadata,
            vec![output],
            vec![public_key.to_string()],
        );

        let private_keys = vec![private_key];

        let signed_transaction = Transaction::sign_transaction(&transaction, private_keys);

        assert_eq!(
            signed_transaction.id.unwrap(),
            "1d050a282da39254bdbec159cba7810d8ab1a46b62793f1287deb4744277e34e"
        );
    }
}
