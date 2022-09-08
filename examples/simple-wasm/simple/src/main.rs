#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use fuels::{
    prelude::{Contract, LocalWallet, Provider, TxParameters},
    signers::wallet::Wallet,
};
use fuels_abigen_macro::abigen;
use std::path::Path;

pub fn tx_params() -> TxParameters {
    let gas_price = 0;
    let gas_limit = 1_000_000;
    let byte_price = 0;
    TxParameters::new(Some(gas_price), Some(gas_limit), Some(byte_price), None)
}

abigen!(
    Simple,
    "examples/simple-wasm/contracts/out/debug/contracts-abi.json"
);

async fn get_contract_id(wallet: &Wallet) -> String {
    debug!("Creating new deployment for non-existent contract");

    let _compiled = Contract::load_sway_contract("../contracts/out/debug/contracts.bin").unwrap();

    let bin_path = "../contracts/out/debug/contracts.bin".to_string();
    let contract_id = Contract::deploy(&bin_path, wallet, tx_params())
        .await
        .unwrap();

    contract_id.to_string()
}

async fn setup_provider_and_wallet(port: u16) -> (Provider, Wallet) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let address = format!("127.0.0.1:{}", port);
    let provider = Provider::connect(address.parse().unwrap()).await.unwrap();

    let path = Path::new(manifest_dir).join("wallet.json");
    let wallet = LocalWallet::load_keystore(&path, "password", Some(provider.clone())).unwrap();

    (provider, wallet)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let (_provider, wallet) = setup_provider_and_wallet(4000).await;
    let contract_id: String = get_contract_id(&wallet).await;
    info!("Using contract at {}", contract_id);
    let contract: Simple = Simple::new(contract_id, wallet);

    let _ = contract.gimme_someevent(7980).call().await;
    let _ = contract.gimme_anotherevent(7890).call().await;

    Ok(())
}