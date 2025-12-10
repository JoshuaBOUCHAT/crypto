use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

use crate::{block_chain::BlockChain, shared::Hash};

mod block_chain;
pub mod blocks;
mod shared;
pub mod transactions;

use ed25519_dalek::{SecretKey, Signer, SigningKey, Verifier, VerifyingKey};

fn main() {
    let mut rng = OsRng {};
    let sign_key = SigningKey::generate(&mut rng);
    sign_key.verifying_key();
    let message = "Salut";
    let a = sign_key.sign(message.as_bytes());
}
