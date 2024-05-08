#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
    sync::{atomic::AtomicUsize, Arc},
};

use solana_sdk::{
    account::{ReadableAccount},
    clock::{Clock, Slot},
    commitment_config::CommitmentConfig,
    keccak::Hash,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::EncodableKey,
    sysvar,
};

use tracing::{debug, error, info, warn};

use tokio::sync::{
    mpsc::{channel, Sender},
    RwLock,
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

pub struct Accounts {
    pub id: usize,
    #[allow(clippy::vec_box)]
    pub signers: Vec<Box<Keypair>>,
    pub pubkey: Vec<Pubkey>,
    pub proof_pda: Vec<Pubkey>,
    // release_stuff: (Sender<Accounts>, Arc<AtomicUsize>),
}

impl Accounts {

    pub const fn size() -> usize {
        3
    }

}



fn main(){
    let key_path = "./keys";

    // Use Box to store
    let all_signers = read_keys(key_path)
        .into_iter()
        .map(Box::new)
        .collect::<Vec<_>>();


    println!("{} keys loaded", all_signers.len());

    let idle_accounts_counter = Arc::new(AtomicUsize::new(all_signers.len()));
   
    println!("{:?}", idle_accounts_counter);

    // Setup channels
    let (ch_accounts, mut ch_accounts_receiver) = channel::<Accounts>(all_signers.len() / Accounts::size());

    // let batches = all_signers
    //     .into_iter()
    //     .chunks(Accounts::size())
    //     .into_iter()
    //     .enumerate()
    //     .map(|(i, signers)| {
    //         let signers = signers.collect::<Vec<_>>();

    //         Accounts {
    //             id: i,
    //             pubkey: signers.iter().map(|k| k.pubkey()).collect(),
    //             proof_pda: signers
    //                 .iter()
    //                 .map(|k| utils::get_proof_pda_no_cache(k.pubkey()))
    //                 .collect(),
    //             signers,
    //             release_stuff: (ch_accounts.clone(), idle_accounts_counter.clone()),
    //         }
    //     })
    //     .collect::<Vec<_>>();

    // for signers in batches {
    //     ch_accounts.send(signers);
    // }

    // info!("splitted signers into batches");
}