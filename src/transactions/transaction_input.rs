use sha2::{Sha256, digest::Update};

use crate::{shared::Hash, transactions::transaction::SignedTransaction};

pub struct Input {
    tx_id: Hash,
    tx_output_idx: usize,
}
impl Input {
    pub fn new(transaction: &SignedTransaction, tx_output_idx: usize) -> Self {
        assert!(
            transaction.outputs().len() > tx_output_idx,
            "transaction output index not in outputs bound"
        );
        Self {
            tx_id: *transaction.get_hash(),
            tx_output_idx,
        }
    }
    pub fn add_to_hash(&self, hasher: &mut Sha256) {
        hasher.update(&self.tx_id);
        hasher.update(&self.tx_output_idx.to_be_bytes());
    }
    pub fn get_tx_id(&self) -> &Hash {
        &self.tx_id
    }
    pub fn get_tx_idx(&self) -> usize {
        self.tx_output_idx
    }
}
