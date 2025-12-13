use std::ops::Index;
use std::ops::IndexMut;

use sha2::Digest;
use sha2::Sha256;

use crate::{shared::Hash, transactions::transaction::ValidatedTransaction};

struct MerkelTree {
    transactions: Vec<ValidatedTransaction>,
}
//cela limit le nombre maximum de transaction en un coup de 1.048.575 soit 2^BUFFER_SIZE -1
const BUFFER_SIZE: usize = 20;

pub fn get_merkel_hash(transactions: &[ValidatedTransaction]) -> Hash {
    assert!(transactions.len() != 0);
    assert!(transactions.len() < ((1 << BUFFER_SIZE) - 1));

    let mut merkel_hash_builder = MerkelHashBuilder::new();
    for hash in transactions
        .iter()
        .map(|transaction| transaction.get_hash())
    {
        merkel_hash_builder.insert_hash(hash);
    }
    merkel_hash_builder.resume_hashs()
}

#[repr(transparent)]
struct MerkelHashBuilder {
    buffer: [Option<Hash>; BUFFER_SIZE],
}
impl Index<usize> for MerkelHashBuilder {
    type Output = Option<Hash>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}
impl IndexMut<usize> for MerkelHashBuilder {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

impl MerkelHashBuilder {
    fn new() -> Self {
        Self {
            buffer: [None; BUFFER_SIZE],
        }
    }
    fn insert_hash(&mut self, hash: &Hash) {
        if let Some(hash2) = &self[0] {
            self[0] = Some(combine_hash(hash2, hash));
        } else {
            self[0] = Some(hash.clone());
            return;
        }

        for i in 1..(BUFFER_SIZE) {
            if let Some(hash2) = &self[i] {
                //can safely unwrap as bubble up because the case contained Some()
                self[i] = Some(combine_hash(hash2, &self[i - 1].unwrap()));
                self[i - 1] = None;
            } else {
                self[i] = self[i - 1];
                self[i - 1] = None;
                break;
            }
        }
    }
    fn resume_hashs(&self) -> Hash {
        let first_index = self.get_first_buffer_index();
        let final_index = self.get_last_buffer_index();
        assert!(
            !(first_index > final_index),
            "Hash builder empty should not be possible"
        );

        let mut hash_accumulator = self[first_index].unwrap();
        for i in (first_index + 1)..final_index {
            let to_add = if let Some(hash) = &self[i] {
                hash
            } else {
                &hash_accumulator
            };
            hash_accumulator = combine_hash(&hash_accumulator, to_add);
        }
        hash_accumulator
    }

    fn get_last_buffer_index(&self) -> usize {
        let mut final_index = 0;
        for i in (0..BUFFER_SIZE).rev() {
            if self[i].is_some() {
                final_index = i;
                break;
            }
        }
        final_index
    }
    fn get_first_buffer_index(&self) -> usize {
        for i in 0..BUFFER_SIZE {
            if self[i].is_some() {
                return i;
            }
        }
        BUFFER_SIZE
    }
}

fn combine_hash(hash1: &Hash, hash2: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(hash1);
    hasher.update(hash2);
    hasher.finalize().into()
}
