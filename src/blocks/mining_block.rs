use sha2::{Digest, Sha256};

use crate::{
    block_chain::BlockChain,
    blocks::{self, block::Block},
    shared::{Hash, count_leading_zeros, get_now_unix},
};

#[derive(Clone, Debug)]
pub struct MiningBlock {
    version: u32,
    difficulty: u32,
    previous_hash: Hash,
    merkel_root: Hash,
    nonce: u64,
    timestamp: u64,
}

impl MiningBlock {
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();

        // sérialisation manuelle (ordre important !!!)
        hasher.update(&self.version.to_be_bytes());
        hasher.update(&self.difficulty.to_be_bytes());
        hasher.update(&self.previous_hash);
        hasher.update(&self.merkel_root);
        hasher.update(&self.nonce.to_be_bytes());
        hasher.update(&self.timestamp.to_be_bytes());

        hasher.finalize().into()
    }
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }
    pub fn get_version(&self) -> u32 {
        self.version
    }
    pub fn get_previous_hash(&self) -> &Hash {
        &self.previous_hash
    }
    pub fn genesis() -> Self {
        Self {
            version: 0,
            difficulty: 24,
            previous_hash: [0; 32],
            merkel_root: [0; 32],
            nonce: 0,
            timestamp: get_now_unix(),
        }
    }
    pub fn mine(&mut self) -> Option<Block> {
        for test_nonce in 0..u64::MAX {
            self.nonce = test_nonce;
            if count_leading_zeros(&self.hash()) >= self.get_difficulty() {
                if let Some(block) = Block::try_new(&self) {
                    return Some(block);
                }
            }
        }
        None
    }
    pub fn from_black_chain(chain: &BlockChain, merkel_root: Hash) -> Self {
        Self {
            version: chain.get_version(),
            difficulty: chain.get_difficulty(),
            previous_hash: chain.get_previous_hash().clone(),
            merkel_root,
            nonce: 0,
            timestamp: get_now_unix(),
        }
    }
}
