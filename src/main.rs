use anyhow::{Context, Result};
use dotenvy::dotenv;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Mint;
use spl_token::solana_program::program_pack::Pack; // <-- for local SPL Mint decoding
use std::env;

/// Try to fetch JsonParsed; if None, fetch raw. If `try_mint_decode` is true,
/// attempt to decode the raw bytes as an SPL Mint for convenience.
fn fetch_parsed_or_raw(rpc: &RpcClient, key: &Pubkey, try_mint_decode: bool) -> Result<()> {
    // Ask for parsed first
    let cfg = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::JsonParsed),
        commitment: None,
        data_slice: None,
        min_context_slot: None,
    };

    let parsed = rpc
        .get_account_with_config(key, cfg)
        .with_context(|| format!("get_account_with_config failed for {key}"))?
        .value;

    if let Some(ui) = parsed {
        println!("Parsed (UI) response:\n{:#?}", ui);
        return Ok(());
    }

    // Fallback: raw bytes
    let raw = rpc
        .get_account(key)
        .with_context(|| format!("get_account (raw) failed for {key}"))?;

    println!(
        "Raw account:\n  lamports={}\n  owner={}\n  executable={}\n  data_len={}\n",
        raw.lamports,
        raw.owner,
        raw.executable,
        raw.data.len()
    );

    // Optional: try to decode as SPL Mint (useful for token mint pubkeys like WSOL)
    if try_mint_decode {
        match Mint::unpack(&raw.data) {
            Ok(m) => {
                println!("Decoded SPL Mint locally:");
                println!("  decimals        : {}", m.decimals);
                println!("  supply          : {}", m.supply);
                println!("  is_initialized  : {}", m.is_initialized);
                println!("  mint_authority  : {:?}", m.mint_authority);
                println!("  freeze_authority: {:?}", m.freeze_authority);
            }
            Err(e) => {
                println!("Raw present but not SPL Mint layout (or not initialized): {e}");
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();

    let rpc = RpcClient::new(
        env::var("HELIUS_RPC_URL").context("HELIUS_RPC_URL not set")?
    );

    // Devnet-safe examples; switch to mainnet + mainnet keys later
    let system_prog: Pubkey = "11111111111111111111111111111111".parse()?;
    let token_prog : Pubkey = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".parse()?;
    let wsol_mint : Pubkey = "So11111111111111111111111111111111111111112".parse()?; // token mint
    let memo_prog  : Pubkey = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr".parse()?;
    let stake_prog : Pubkey = "Stake11111111111111111111111111111111111111".parse()?;

    // (name, key, try_mint_decode)
    let items = [
        ("System Program", system_prog, false),
        ("SPL Token Program", token_prog, false),
        ("WSOL Mint", wsol_mint, true), // <- decode as SPL Mint if JsonParsed is null
        ("Memo Program", memo_prog, false),
        ("Stake Program", stake_prog, false),
    ];

    for (name, key, try_mint) in items {
        println!("=== {name} ({key}) ===");
        if let Err(e) = fetch_parsed_or_raw(&rpc, &key, try_mint) {
            eprintln!("{name} fetch failed: {e}\n");
        }
        println!();
    }

    Ok(())
}
