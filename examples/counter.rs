//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const PRIV_KEY_PATH: &str = "privkey.txt";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "https://stylus-testnet.arbitrum.io/rpc";

/// Deployed pragram address.
const STYLUS_PROGRAM_ADDRESS: &str = "0x8A99643dcF67bE8a94b6F8d7dA21B8634b2fa44E";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // We are not reading from ENV
    // let priv_key_path =
    // std::env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;

    // let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    // let program_address = std::env::var(STYLUS_PROGRAM_ADDRESS)
    //     .map_err(|_| eyre!("No {} env var set", STYLUS_PROGRAM_ADDRESS))?;
    abigen!(
        Counter,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 number) external
            function increment() external
        ]"#
    );

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let address: Address = STYLUS_PROGRAM_ADDRESS.parse()?;

    let privkey = read_secret_from_file(PRIV_KEY_PATH)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let counter = Counter::new(address, client);
    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num);

    let _ = counter.increment().send().await?.await?;
    println!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    println!("New counter number value = {:?}", num);
    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
