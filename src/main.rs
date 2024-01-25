use std::collections::HashMap;

use bigchaindb::{connection::Connection, json::json, transaction::Transaction};

#[tokio::main]
async fn main() {
    // let nodes = "http://localhost:3000/".to_string();
    let nodes = "http://198.19.249.99:9984/api/v1/".to_string();
    let mut conn = Connection::new(vec![Some(nodes)], HashMap::new(), None);

    let assetdata = json!({
        "ft": {
            "signature": "rust",
            "device": "rust",
        }
    });
    let metadata = json!({"metadata": "rust"});
    let public_key = "6zaQbbRi7RCFhCF35tpVDu2nEfR9fZCqx2MvUa7pKRmX";
    let private_key = "CHwxsNPzRXTzCz25KZ9TJcBJ45H25JKkLL4HrX1nBfXT";
    let condition = Transaction::make_ed25519_condition(&public_key, true).unwrap();
    let output = Transaction::make_output(condition, String::from("1"));
    let transaction = Transaction::make_create_transaction(
        Some(assetdata),
        metadata,
        vec![output],
        vec![public_key.to_string()],
    );

    let signed_transaction = Transaction::sign_transaction(&transaction, vec![private_key]);
    let tx = conn
        .post_transaction_commit(signed_transaction)
        .await
        .unwrap();

    println!("{:?}", tx);
}
