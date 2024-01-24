use std::collections::HashMap;

use bigchaindb::{
    connection::Connection,
    transaction::{AssetDefinition, Operation, TransactionTemplate},
};

#[tokio::main]
async fn main() {
    let nodes = "http://localhost:3000/".to_string();
    let mut conn = Connection::new(vec![Some(nodes)], HashMap::new(), None);

    let asset = serde_json::json!({
        "ft": {
            "signature": "signature",
            "device": "device",
        }
    });
    let metadata = serde_json::json!({"metadata": "metadata"});

    let asset = AssetDefinition { data: Some(asset) };

    let mut transaction = TransactionTemplate::new();
    transaction.asset = Some(asset);
    transaction.metadata = Some(metadata);
    transaction.operation = Some(Operation::CREATE);

    let _ = conn.post_transaction_commit(transaction).await;
}
