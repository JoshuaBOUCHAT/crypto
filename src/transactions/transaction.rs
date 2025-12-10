
use ed25519_dalek::{Signature, SigningKey, VerifyingKey, ed25519::signature::SignerMut};
use sha2::{Digest, Sha256};

use crate::{
    shared::Hash,
    transactions::{transaction_input::Input, transaction_output::Output},
    utxo_map::{ UTXOMap},
};

pub struct RawTransaction {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    pubkey:VerifyingKey
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
}

pub struct ValidatedTransaction {}

pub enum TransactionValidationError {
    InsufficientInput,
    InsufficientOutput,
    InputInvalid,
    UnauthorizedInput
}

impl ValidatedTransaction {
    pub fn validate(
        signed_transaction: SignedTransaction,
        utxo_map: UTXOMap,
    ) -> Result<Self, TransactionValidationError> {
        for input in signed_transaction.inputs() {
            let Some(output) = utxo_map.try_find_matching_output(input) else {
                return Err(TransactionValidationError::InputInvalid);
            };
            if output.get_pubkey()!=signed_transaction.signature.
        }
    }
}
