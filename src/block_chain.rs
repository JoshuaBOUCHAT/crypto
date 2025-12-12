use ed25519_dalek::SigningKey;

use crate::{
    blocks::{
        block::{Block, BlockValidationError},
        mining_block::MiningBlock,
    },
    shared::Hash,
    transactions::{merkel::get_merkel_hash, transaction::ValidatedTransaction},
    utxo_map::UTXOMap,
};

pub struct BlockChain {
    blocks: Vec<Block>,
    utxo: UTXOMap,
    difficulty: u32,
    version: u32,
}
impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            difficulty: 20,
            version: 0,
            utxo: UTXOMap::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    pub fn peak<'a>(&'a self) -> &'a Block {
        &self.blocks[0]
    }
    pub fn get_coin_base_amount(&self) -> u64 {
        todo!()
    }
    pub fn get_mining_block(
        &self,
        transactions: &[ValidatedTransaction],
        sign_key: &mut SigningKey,
    ) -> MiningBlock {
        let coin_base = ValidatedTransaction::get_coin_base(&self, sign_key);
        let merkel_root = get_merkel_hash(transactions, coin_base);
        MiningBlock::from_black_chain(&self, merkel_root)
    }
    pub fn get_version(&self) -> u32 {
        todo!()
    }
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }
    pub fn get_previous_hash(&self) -> &Hash {
        self.peak().get_hash()
    }
    pub fn check_compatibility(
        &self,
        mining_block: &MiningBlock,
    ) -> Result<(), BlockValidationError> {
        if self.get_difficulty() > mining_block.get_difficulty() {
            return Err(BlockValidationError::DifficultyTooLow);
        }
        if self.version > mining_block.get_version() {
            return Err(BlockValidationError::VersionTooLow);
        }
        if self.peak().get_hash() != mining_block.get_previous_hash() {
            return Err(BlockValidationError::WrongPreviousHash);
        }
        Ok(())
    }
    pub fn get_utxo(&self) -> &UTXOMap {
        &self.utxo
    }
}
