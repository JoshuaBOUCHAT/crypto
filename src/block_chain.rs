use crate::blocks::{block::Block, mining_block::MiningBlock};

pub struct BlockChain {
    blocks: Vec<Block>,
}
impl BlockChain {
    pub fn new() -> Self {
        let initial_block = MiningBlock::genesis().mine().unwrap();
        Self {
            blocks: vec![initial_block],
        }
    }
    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    pub fn peak<'a>(&'a self) -> &'a Block {
        &self.blocks[0]
    }
}
