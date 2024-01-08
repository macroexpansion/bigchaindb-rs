use bigchaindb;

fn main() {
    let keypair = bigchaindb::ed25519_keypair();
    println!("{keypair:?}")
}
