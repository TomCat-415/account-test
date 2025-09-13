use anyhow::{Context, Result};
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::pubkey::Pubkey;

fn fetch_parsed(rpc: &RpcClient, key: &Pubkey) -> Result<String> {
    let cfg = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::JsonParsed),
        commitment: None,
        data_slice: None,
        min_context_slot: None,
    };
    let ui = rpc
        .get_account_with_config(key, cfg)
        .with_context(|| format!("RPC get_account_with_config failed for {key}"))?
        .value
        .ok_or_else(|| anyhow::anyhow!("account not found"))?;
    Ok(format!("{:#?}", ui))
}

fn main() -> Result<()> {
    // 1) Primary + fallback RPCs (public; for serious work use a dedicated provider)
    let prim = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let alt  = RpcClient::new("https://rpc.ankr.com/solana".to_string());

    // 2) Known-good accounts to test:
    // - System Program
    let system_prog: Pubkey = "11111111111111111111111111111111".parse()?;
    // - Token Program (SPL Token)
    let token_prog: Pubkey = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".parse()?;
    // - USDC Mint (sometimes flaky on public RPCs)
    let usdc_mint: Pubkey = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".parse()?;

    for (name, key) in [
        ("System Program", system_prog),
        ("SPL Token Program", token_prog),
        ("USDC Mint", usdc_mint),
    ] {
        println!("=== {name} ({key}) ===");
        let out = fetch_parsed(&prim, &key)
            .or_else(|e| {
                eprintln!("Primary RPC said: {e}. Trying fallback RPC…");
                fetch_parsed(&alt, &key)
            });
        match out {
            Ok(text) => println!("{text}"),
            Err(e) => eprintln!("Both RPCs failed for {name}: {e}"),
        }
        println!();
    }

    Ok(())
}