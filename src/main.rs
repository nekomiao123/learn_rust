use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use solana_sdk::{
    account::{Account, ReadableAccount},
    clock::{Clock, Slot},
    commitment_config::CommitmentConfig,
    keccak::Hash,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::EncodableKey,
    sysvar,
};


pub fn read_keys(key_folder: &str) -> Vec<Keypair> {
    fs::read_dir(key_folder)
        .expect("Failed to read key folder")
        .map(|entry| {
            let path = entry.expect("Failed to read entry").path();

            Keypair::read_from_file(&path).unwrap_or_else(|_| panic!("Failed to read keypair from {:?}", path))
        })
        .collect::<Vec<_>>()
}


fn main(){
    let key_path = "./keys";
    println!("Hello, world!");
    let all_signers = read_keys(key_path)
        .into_iter()
        .map(Box::new)
        .collect::<Vec<_>>();

    println!("all_signers: {:?}", all_signers);
}