use solana_client::nonblocking::rpc_client::RpcClient;
use solana_account_decoder::UiAccountEncoding;
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    // RPC endpoint (can use free public RPC to start)
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // Example: USDC mint account on Solana
    let usdc_mint: Pubkey = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        .parse()
        .unwrap();

    // Fetch account
    let acct = client
        .get_account_with_commitment(&usdc_mint, solana_client::rpc_config::RpcCommitmentConfig::confirmed())
        .await
        .unwrap();

    // Decode account into JSON-friendly format
    let ui_acct = solana_account_decoder::UiAccount::encode(
        &usdc_mint,
        &acct.value.unwrap(),
        UiAccountEncoding::JsonParsed,
    );

    println!("{:#?}", ui_acct);
}