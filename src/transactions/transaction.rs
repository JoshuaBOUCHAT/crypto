use ed25519_dalek::{Signature, SigningKey, VerifyingKey, ed25519::signature::SignerMut};
use sha2::{Digest, Sha256};

use crate::{
    block_chain::BlockChain,
    shared::Hash,
    transactions::{transaction_input::Input, transaction_output::Output},
    utxo_map::UTXOMap,
};

pub struct RawTransaction {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    pubkey: VerifyingKey,
}
impl RawTransaction {
    pub fn inputs(&self) -> &[Input] {
        &self.inputs
    }
    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        for input in &self.inputs {
            input.add_to_hash(&mut hasher);
        }
        for output in &self.outputs {
            output.add_to_hash(&mut hasher);
        }
        hasher.update(self.pubkey.as_bytes());
        hasher.finalize().into()
    }
    pub fn sign(self, sign_key: &mut SigningKey) -> SignedTransaction {
        SignedTransaction::from_raw(self, sign_key)
    }
    pub fn get_pubkey(&self) -> &VerifyingKey {
        &self.pubkey
    }
    fn coinbase(pubkey: VerifyingKey, amount: u64) -> Self {
        Self {
            inputs: vec![],
            outputs: vec![Output::new(pubkey, amount)],
            pubkey,
        }
    }
}

pub struct SignedTransaction {
    raw: RawTransaction,
    hash: Hash,
    signature: Signature,
}
impl SignedTransaction {
    fn data(&self) -> &RawTransaction {
        &self.raw
    }

    pub fn inputs(&self) -> &[Input] {
        self.raw.inputs()
    }
    pub fn outputs(&self) -> &[Output] {
        self.raw.outputs()
    }
    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }
    fn from_raw(raw: RawTransaction, sign_key: &mut SigningKey) -> Self {
        let hash = raw.hash();
        let signature = sign_key.sign(&hash);
        Self {
            raw,
            hash,
            signature,
        }
    }
    fn get_pubkey(&self) -> &VerifyingKey {
        self.raw.get_pubkey()
    }
    fn check_signature(&self) -> Result<(), TransactionValidationError> {
        if self.raw.hash() != self.hash {
            return Err(TransactionValidationError::HashIncorrect);
        }
        match self.get_pubkey().verify_strict(&self.hash, &self.signature) {
            Ok(()) => Ok(()),
            Err(_) => Err(TransactionValidationError::SignatureIncorrect),
        }
    }
    fn coinbase(sign_key: &mut SigningKey, amount: u64) -> Self {
        let raw = RawTransaction::coinbase(sign_key.verifying_key(), amount);
        Self::from_raw(raw, sign_key)
    }
}

#[repr(transparent)]
pub struct ValidatedTransaction {
    transaction: SignedTransaction,
}

pub enum TransactionValidationError {
    SignatureIncorrect,
    HashIncorrect,
    InsufficientInput,
    InsufficientOutput,
    InputInvalid,
    UnauthorizedInput,
}

impl ValidatedTransaction {
    pub fn validate(
        signed_transaction: SignedTransaction,
        utxo_map: &UTXOMap,
    ) -> Result<Self, TransactionValidationError> {
        let total_input = Self::sum_and_validat_inputs(
            signed_transaction.inputs(),
            signed_transaction.get_pubkey(),
            utxo_map,
        )?;
        let total_output: u64 = signed_transaction
            .outputs()
            .iter()
            .map(|output| output.get_amount())
            .sum();
        Self::check_balance(total_input, total_output)?;
        signed_transaction.check_signature()?;

        Ok(Self {
            transaction: signed_transaction,
        })
    }
    fn sum_and_validat_inputs(
        inputs: &[Input],
        pubkey: &VerifyingKey,
        utxo_map: &UTXOMap,
    ) -> Result<u64, TransactionValidationError> {
        let mut input_sum = 0;
        for input in inputs {
            let Some(output) = utxo_map.try_find_matching_output(input) else {
                return Err(TransactionValidationError::InputInvalid);
            };
            if output.get_pubkey() != pubkey {
                return Err(TransactionValidationError::UnauthorizedInput);
            }
            input_sum += output.get_amount()
        }
        Ok(input_sum)
    }
    fn check_balance(
        total_input: u64,
        total_output: u64,
    ) -> Result<(), TransactionValidationError> {
        use std::cmp::Ordering;
        match total_input.cmp(&total_output) {
            Ordering::Equal => Ok(()),
            Ordering::Less => Err(TransactionValidationError::InsufficientInput),
            Ordering::Greater => Err(TransactionValidationError::InsufficientOutput),
        }
    }
    pub fn get_hash(&self) -> &Hash {
        self.transaction.get_hash()
    }
    pub fn get_coin_base(block_chain: &BlockChain, sign_key: &mut SigningKey) -> Self {
        let amount = block_chain.get_coin_base_amount();

        Self {
            transaction: SignedTransaction::coinbase(sign_key, amount),
        }
    }
    pub fn inputs(&self) -> &[Input] {
        self.transaction.inputs()
    }
}
