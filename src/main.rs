use std::collections::HashMap;

use bigchaindb::{
    connection::Connection,
    json::json,
    transaction::{AssetDefinition, Operation, TransactionTemplate},
};

#[tokio::main]
async fn main() {
    let nodes = "http://localhost:3000/".to_string();
    let mut conn = Connection::new(vec![Some(nodes)], HashMap::new(), None);

    let asset = json!({
        "ft": {
            "signature": "signature",
            "device": "device",
        }
    });
    let metadata = json!({"metadata": "metadata"});

    let asset = AssetDefinition { data: Some(asset) };

    let mut transaction = TransactionTemplate::new();
    transaction.asset = Some(asset);
    transaction.metadata = Some(metadata);
    transaction.operation = Some(Operation::CREATE);

    let _ = conn.post_transaction_commit(transaction).await;
}
