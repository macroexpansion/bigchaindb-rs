#![allow(dead_code)]

use bigchaindb::{connection::Connection, json::json, transaction::Transaction};

#[tokio::main]
async fn main() {
    test_list_outputs().await;
    test_get_transaction().await;
}

async fn test_post_transaction_commit() {
    let nodes = "http://localhost:3000/";
    // let nodes = "http://198.19.249.99:9984/api/v1/";
    let mut conn = Connection::new(vec![nodes]);

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

async fn test_list_outputs() {
    let nodes = "http://localhost:3000/";
    let mut conn = Connection::new(vec![nodes]);
    let tx = conn.list_outputs("fdfdsfdsfsa", None).await.unwrap();
    println!("{:?}", tx);
}

async fn test_get_transaction() {
    let nodes = "http://localhost:3000/";
    let mut conn = Connection::new(vec![nodes]);
    let tx = conn.get_transaction("1").await.unwrap();
    println!("{:?}", tx);
}
