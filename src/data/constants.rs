use std::{str::FromStr, sync::RwLock};

use bitcoin::Network;
use once_cell::sync::Lazy;

fn dot_env(key: &str) -> String {
    let env_file = include_str!("../../.env");
    if let Some(line) = env_file.split('\n').find(|e| e.starts_with(key)) {
        let (_, val) = line.split_once('=').expect("value exists for key");
        val.to_owned()
    } else {
        panic!("Couldn't access .env key: {}", key);
    }
}

static BITCOIN_EXPLORER_API_MAINNET: Lazy<String> =
    Lazy::new(|| dot_env("BITCOIN_EXPLORER_API_MAINNET"));
static BITCOIN_EXPLORER_API_TESTNET: Lazy<String> =
    Lazy::new(|| dot_env("BITCOIN_EXPLORER_API_TESTNET"));
static BITCOIN_EXPLORER_API_SIGNET: Lazy<String> =
    Lazy::new(|| dot_env("BITCOIN_EXPLORER_API_SIGNET"));
pub static BITCOIN_EXPLORER_API: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new(BITCOIN_EXPLORER_API_TESTNET.to_owned()) //TODO: Change default to mainnet
});
pub static NODE_SERVER_BASE_URL: Lazy<String> = Lazy::new(|| dot_env("NODE_SERVER_BASE_URL"));

// Keys local storage
pub const STORAGE_KEY_DESCRIPTOR_ENCRYPTED: &str = "descriptor";
pub const STORAGE_KEY_UNSPENTS: &str = "unspents";
pub const STORAGE_KEY_TRANSACTIONS: &str = "transactions";
pub const STORAGE_KEY_BLINDED_UNSPENTS: &str = "blinded_unspents";

// Descriptor strings
pub const STRING_DESCRIPTOR: &str = "m/84h/1h/0h/0";
pub const STRING_CHANGE_DESCRIPTOR: &str = "m/84h/1h/0h/1";

pub static NETWORK: Lazy<RwLock<Network>> = Lazy::new(|| {
    RwLock::new(Network::Testnet) // TODO: Change default to mainnet
});

// See: https://docs.rs/bitcoin/0.27.1/src/bitcoin/network/constants.rs.html#62-75
pub fn switch_network(network_str: &str) {
    let network = Network::from_str(network_str).unwrap();

    *BITCOIN_EXPLORER_API.write().unwrap() = match network {
        Network::Bitcoin => BITCOIN_EXPLORER_API_MAINNET.to_owned(),
        Network::Testnet => BITCOIN_EXPLORER_API_TESTNET.to_owned(),
        Network::Signet => BITCOIN_EXPLORER_API_SIGNET.to_owned(),
        Network::Regtest => unimplemented!(),
    };

    *NETWORK.write().unwrap() = network;
}
