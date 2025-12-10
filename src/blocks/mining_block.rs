use sha2::{Digest, Sha256};

use crate::{
    blocks::block::Block,
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
}
