use std::collections::{HashMap, HashSet};

use crate::{
    block_chain::BlockChain,
    blocks::mining_block::MiningBlock,
    shared::{Hash, count_leading_zeros, meet_difficulty},
    transactions::transaction::{
        SignedTransaction, TransactionValidationError, ValidatedTransaction,
    },
    utxo_map::UTXOMap,
};

pub struct UntrustedBlock {
    data: MiningBlock,
    hash: Hash,
    transactions: Vec<SignedTransaction>,
}

pub struct Block {
    data: MiningBlock,
    hash: Hash,
    transactions: Vec<ValidatedTransaction>,
}
impl Block {
    pub fn try_new(mining_block: &MiningBlock) -> Option<Self> {
        let hash = mining_block.hash();
        if count_leading_zeros(&hash) >= mining_block.get_difficulty() {
            Some(Self {
                data: mining_block.clone(),
                hash,
                transactions: vec![],
            })
        } else {
            None
        }
    }
    pub fn get_mining(&self) -> &MiningBlock {
        &self.data
    }
    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }
    pub fn valid_new_block(
        chain: &BlockChain,
        untrusted_block: UntrustedBlock,
    ) -> Result<Block, BlockValidationError> {
        if untrusted_block.data.hash() != untrusted_block.hash {
            return Err(BlockValidationError::WrongHash);
        }

        chain.check_compatibility(&untrusted_block.data)?;

        let utxo = chain.get_utxo();
        let validated_transactions =
            Self::validate_untrusted_transaction(untrusted_block.transactions, utxo)?;

        let new_block = Self {
            data: untrusted_block.data,
            hash: untrusted_block.hash,
            transactions: validated_transactions,
        };
        Ok(new_block)
    }
    fn validate_untrusted_transaction(
        untrusted_signed_transaction: Vec<SignedTransaction>,
        utxo: &UTXOMap,
    ) -> Result<Vec<ValidatedTransaction>, BlockValidationError> {
        let mut spent_map: HashSet<(Hash, usize)> = HashSet::new();
        let mut validated_transactions = Vec::with_capacity(untrusted_signed_transaction.len());
        for untrused_transaction in untrusted_signed_transaction {
            let is_input_already_spent = untrused_transaction
                .inputs()
                .iter()
                .any(|input| spent_map.contains(&(*input.get_tx_id(), input.get_tx_idx())));
            if is_input_already_spent {
                return Err(BlockValidationError::UTXOSpentMultipleTime);
            }
            match ValidatedTransaction::validate(untrused_transaction, utxo) {
                Ok(valid_transaction) => {
                    for input in valid_transaction.inputs() {
                        spent_map.insert((input.get_tx_id().clone(), input.get_tx_idx()));
                    }
                    validated_transactions.push(valid_transaction);
                }
                Err(err) => return Err(BlockValidationError::TransactionValidationError(err)),
            }
        }
        Ok(validated_transactions)
    }
}
pub enum BlockValidationError {
    DifficultyTooLow,
    VersionTooLow,
    WrongPreviousHash,
    WrongHash,
    UTXOSpentMultipleTime,
    TransactionValidationError(TransactionValidationError),
}
