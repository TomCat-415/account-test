use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcCommitmentConfig;
use solana_account_decoder::UiAccountEncoding;
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // USDC mint, easy to test
    let usdc_mint: Pubkey = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".parse()?;

    let acct = client
        .get_account_with_commitment(&usdc_mint, RpcCommitmentConfig::confirmed())
        .await?
        .value
        .ok_or_else(|| anyhow::anyhow!("account not found"))?;

    let ui = solana_account_decoder::UiAccount::encode(
        &usdc_mint,
        &acct,
        UiAccountEncoding::JsonParsed,
    );

    println!("{:#?}", ui);
    Ok(())
}