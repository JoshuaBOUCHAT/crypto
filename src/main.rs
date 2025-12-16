use std::time::Instant;

use rand::rngs::OsRng;

use crate::{block_chain::BlockChain, transactions::transaction::ValidatedTransaction};

mod block_chain;
pub mod blocks;
mod shared;
pub mod transactions;
pub mod utxo_map;

use ed25519_dalek::SigningKey;

fn main() {
    let mut rng = OsRng {};
    let mut sign_key = SigningKey::generate(&mut rng);
    let mut block_chain = BlockChain::new();
    let coin_base = ValidatedTransaction::get_coin_base(&block_chain, &mut sign_key);
    let transactions = [coin_base];
    let mining_block = block_chain.get_mining_block(&transactions);
    let now = Instant::now();
    if let Some(mined_block) = mining_block.mine_multithread() {
        block_chain.update(mined_block);
        println!("Block miné en {}s", now.elapsed().as_secs_f32());
        println!("Voici le block miné:\n{}", block_chain.peak())
    } else {
        println!("truc")
    }
}
