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
    utxos: UTXOMap,
    difficulty: u32,
    version: u32,
}
impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            difficulty: 20,
            version: 0,
            utxos: UTXOMap::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    pub fn peak<'a>(&'a self) -> &'a Block {
        &self.blocks[0]
    }
    pub fn get_coin_base_amount(&self) -> u64 {
        1_000_000
    }
    pub fn get_mining_block(&self, transactions: &[ValidatedTransaction]) -> MiningBlock {
        assert!(transactions.len() != 0);

        let merkel_root = get_merkel_hash(transactions);
        MiningBlock::from_black_chain(&self, merkel_root)
    }
    pub fn get_version(&self) -> u32 {
        self.version
    }
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }
    pub fn get_previous_hash(&self) -> Hash {
        if self.blocks.len() == 0 {
            Hash::default()
        } else {
            self.peak().get_hash().clone()
        }
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
    pub fn get_utxos(&self) -> &UTXOMap {
        &self.utxos
    }
    pub fn update(&mut self, block: Block) {
        for tx in block.get_transactions() {
            self.utxos.update_transaction(tx);
        }
        self.blocks.push(block);
    }
}
