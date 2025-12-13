use std::collections::HashMap;

use sha2::{Digest, Sha256};

use crate::{
    shared::Hash,
    transactions::{
        transaction::ValidatedTransaction, transaction_input::Input, transaction_output::Output,
    },
};

pub struct UTXOMap {
    utxos: HashMap<Input, Output>,
}

impl UTXOMap {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
        }
    }
    pub fn update_transaction(&mut self, transaction: &ValidatedTransaction) {
        self.remove_utxos(transaction.inputs());
        self.add_utxos(transaction.outputs(), transaction.get_hash());
    }
    pub fn try_find_matching_output(&self, input: &Input) -> Option<&Output> {
        self.utxos.get(input)
    }
    fn remove_utxos(&mut self, inputs: &[Input]) {
        for input in inputs {
            self.utxos.remove(input);
        }
    }
    fn add_utxos(&mut self, outputs: &[Output], tx_id: &Hash) {
        for (tx_output_idx, output) in outputs.iter().enumerate() {
            let utxo_key = Input::new(tx_id.clone(), tx_output_idx);
            self.utxos.insert(utxo_key, output.clone());
        }
    }
}
