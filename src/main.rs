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
    let mut mining_block = block_chain.get_mining_block(&transactions);
    if let Some(mined_block) = mining_block.mine() {
        block_chain.update(mined_block);
        println!("Trop bien t'es riche")
    } else {
        println!("truc")
    }
}
