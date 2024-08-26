use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let pubkey = Pubkey::from_str("YourPublicKeyHere").unwrap();

    let balance = client.get_balance(&pubkey).unwrap();
    println!("Balance: {}", balance);
}
