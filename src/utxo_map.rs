use std::collections::HashMap;

use crate::{
    shared::Hash,
    transactions::{
        transaction::ValidatedTransaction, transaction_input::Input, transaction_output::Output,
    },
};

pub struct UTXOMap {
    utxos: HashMap<Hash, Vec<Output>>,
}
impl UTXOMap {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
        }
    }
    pub fn update_transaction(&mut self, transaction: &ValidatedTransaction) {
        todo!()
    }
    pub fn try_find_matching_output(&self, input: &Input) -> Option<&Output> {
        Some(&self.utxos.get(input.get_tx_id())?[input.get_tx_idx()])
    }
}
