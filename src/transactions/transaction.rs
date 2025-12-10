use ed25519_dalek::{Signature, SigningKey, ed25519::signature::SignerMut};
use sha2::{Digest, Sha256};

use crate::{
    shared::Hash,
    transactions::{transaction_input::Input, transaction_output::Output},
};

pub struct RawTransaction {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
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
