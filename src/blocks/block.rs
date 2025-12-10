use crate::{
    blocks::mining_block::MiningBlock,
    shared::{Hash, count_leading_zeros},
};

#[derive(Debug)]
pub struct Block {
    data: MiningBlock,
    hash: Hash,
}
impl Block {
    pub fn try_new(mining_block: &MiningBlock) -> Option<Self> {
        let hash = mining_block.hash();
        if count_leading_zeros(&hash) >= mining_block.get_difficulty() {
            Some(Self {
                data: mining_block.clone(),
                hash,
            })
        } else {
            None
        }
    }
    pub fn get_mining(&self) -> &MiningBlock {
        &self.data
    }
}
