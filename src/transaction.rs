use std::collections::HashSet;

use crypto_conditions::{self, fulfillment::Fulfillment, Ed25519Sha256};
use serde::Serialize;

use crate::{cc_jsonify, Details, JsonBody};

#[derive(Debug, Serialize)]
pub enum Operation {
    CREATE,
    TRANSFER,
}

#[derive(Debug, Serialize)]
pub struct AssetDefinition<T: Serialize> {
    pub data: Option<T>,
}

/// Fields of this struct needed to be sorted alphabetically
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct Output {
    pub amount: String,
    pub condition: JsonBody,
    pub public_keys: Vec<String>,
}

/// Fields of this struct needed to be sorted alphabetically
#[derive(Debug, Serialize)]
pub struct TransactionTemplate<M: Serialize, A: Serialize> {
    pub asset: Option<A>,
    pub id: Option<String>,
    pub inputs: Vec<InputTemplate>,
    pub metadata: Option<M>,
    pub operation: Option<Operation>,
    pub outputs: Vec<Output>,
    pub version: String,
}

impl<M: Serialize, A: Serialize> TransactionTemplate<M, A> {
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
        asset: impl Serialize,
        metadata: impl Serialize,
        outputs: Vec<Output>,
        inputs: Vec<InputTemplate>,
    ) -> TransactionTemplate<impl Serialize, impl Serialize> {
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
        asset: Option<impl Serialize>,
        metadata: impl Serialize,
        outputs: Vec<Output>,
        issuers: Vec<String>,
    ) -> TransactionTemplate<impl Serialize, impl Serialize> {
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
}
