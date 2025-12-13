use ed25519_dalek::VerifyingKey;
use sha2::{Sha256, digest::Update};

#[derive(Clone)]
pub struct Output {
    pubkey: VerifyingKey,
    amount: u64,
}
impl Output {
    pub fn new(pubkey: VerifyingKey, amount: u64) -> Self {
        Output { pubkey, amount }
    }

    pub fn get_pubkey(&self) -> &VerifyingKey {
        &self.pubkey
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }
    pub fn add_to_hash(&self, hasher: &mut Sha256) {
        hasher.update(self.pubkey.as_bytes());
        hasher.update(&self.amount.to_be_bytes());
    }
}
