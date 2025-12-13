use std::collections::HashMap;

use sha2::{Digest, Sha256};

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
    pub fn update_transaction(
        &mut self,
        transaction: &ValidatedTransaction,
    ) -> Result<(), UTXOUpdateError> {
        for input in transaction.inputs() {
            self.remove_utxo(input);
        }
    }
    pub fn try_find_matching_output(&self, input: &Input) -> Option<&Output> {
        self.utxos.get(input.get_tx_id())?.get(input.get_tx_idx())
    }
    fn remove_utxo(&mut self, input: &Input) {
        self.utxos
            .get_mut(input.get_tx_id())
            .expect("try to spend non existent uxto")
            .swap_remove(input.get_tx_idx());
    }
    fn add_utxo(&mut self, output: Output,transaction_id:&Hash) {
        if let S
    }
}
