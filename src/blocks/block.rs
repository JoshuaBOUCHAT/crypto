use std::collections::HashSet;

use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{
    block_chain::BlockChain,
    blocks::mining_block::MiningBlock,
    shared::{Hash, meet_difficulty},
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
        if meet_difficulty(&hash, mining_block.get_difficulty()) {
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

        let utxo = chain.get_utxos();
        let validated_transactions =
            validate_untrusted_transactions(untrusted_block.transactions, utxo)?;

        let new_block = Self {
            data: untrusted_block.data,
            hash: untrusted_block.hash,
            transactions: validated_transactions,
        };
        Ok(new_block)
    }

    pub fn get_transactions(&self) -> &[ValidatedTransaction] {
        &self.transactions
    }
}

fn validate_untrusted_transactions(
    untrusted_signed_transactions: Vec<SignedTransaction>,
    utxos: &UTXOMap,
) -> Result<Vec<ValidatedTransaction>, BlockValidationError> {
    let mut spent_map: HashSet<(Hash, usize)> = HashSet::new();
    untrusted_signed_transactions
        .into_iter()
        .map(|tx| validate_untrusted_transaction(utxos, &mut spent_map, tx))
        .collect::<Result<Vec<ValidatedTransaction>, BlockValidationError>>()
}

fn validate_untrusted_transaction(
    utxos: &UTXOMap,
    spent_map: &mut HashSet<(Hash, usize)>,
    untrusted_transaction: SignedTransaction,
) -> Result<ValidatedTransaction, BlockValidationError> {
    let is_input_already_spent = untrusted_transaction
        .inputs()
        .iter()
        .any(|input| spent_map.contains(&(*input.get_tx_id(), input.get_tx_idx())));
    if is_input_already_spent {
        return Err(BlockValidationError::UTXOSpentMultipleTime);
    }

    let valid_transaction = ValidatedTransaction::validate(untrusted_transaction, utxos)
        .map_err(|err| BlockValidationError::TransactionValidationError(err))?;
    for input in valid_transaction.inputs() {
        spent_map.insert((input.get_tx_id().clone(), input.get_tx_idx()));
    }
    Ok(valid_transaction)
}

pub enum BlockValidationError {
    DifficultyTooLow,
    VersionTooLow,
    WrongPreviousHash,
    WrongHash,
    UTXOSpentMultipleTime,
    TransactionValidationError(TransactionValidationError),
}
impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n hash:{}",
            self.get_mining(),
            BASE64_STANDARD.encode(&self.hash),
        )
    }
}
