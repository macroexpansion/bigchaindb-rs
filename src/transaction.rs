use bs58;
use crypto_conditions::{self, Ed25519Sha256};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Operation {
    CREATE,
    TRANSFER,
}

#[derive(Debug, Serialize)]
pub struct AssetDefinition<T: Serialize> {
    pub data: Option<T>,
}

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

#[derive(Debug, Serialize)]
pub struct Output {
    pub condition: Ed25519Condition,
    pub amount: String,
    pub public_keys: Vec<String>,
}

impl Output {
    pub fn new(condition: Ed25519Condition, amount: String) -> Self {
        Self {
            condition,
            amount,
            public_keys: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TransactionTemplate<M: Serialize, A: Serialize> {
    pub id: Option<String>,
    pub operation: Option<Operation>,
    pub outputs: Vec<Output>,
    pub inputs: Vec<InputTemplate>,
    pub metadata: Option<M>,
    pub asset: Option<A>,
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
        let tx = Self::make_transaction(
            Operation::CREATE,
            asset_definition,
            metadata,
            outputs,
            inputs,
        );
        tx
    }
}
