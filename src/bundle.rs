use std::{
    sync::{atomic::AtomicUsize, Arc}, time::{Duration, Instant}
};

pub fn bundle(args: Args) {
    let all_signers = Self::read_keys(&args.key_folder)
            .into_iter()
            .map(Box::new)
            .collect::<Vec<_>>();

        if all_signers.len() % Accounts::size() != 0 {
            panic!("number of keys must be a multiple of {}", Accounts::size());
        }

        info!("{} keys loaded", all_signers.len());

        let idle_accounts_counter = Arc::new(AtomicUsize::new(all_signers.len()));

        // Setup channels
        let (ch_accounts, mut ch_accounts_receiver) = channel::<Accounts>(all_signers.len() / Accounts::size());

        let batches = all_signers
            .into_iter()
            .chunks(Accounts::size())
            .into_iter()
            .enumerate()
            .map(|(i, signers)| {
                let signers = signers.collect::<Vec<_>>();

                Accounts {
                    id: i,
                    pubkey: signers.iter().map(|k| k.pubkey()).collect(),
                    proof_pda: signers
                        .iter()
                        .map(|k| utils::get_proof_pda_no_cache(k.pubkey()))
                        .collect(),
                    signers,
                    release_stuff: (ch_accounts.clone(), idle_accounts_counter.clone()),
                }
            })
            .collect::<Vec<_>>();

        for signers in batches {
            ch_accounts.send(signers).await.unwrap();
        }

        info!("splitted signers into batches");
}