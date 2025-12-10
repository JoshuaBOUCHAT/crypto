use std::time::{SystemTime, UNIX_EPOCH};

pub type Hash = [u8; 32];

pub fn get_now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
pub fn count_leading_zeros(hash: &[u8; 32]) -> u32 {
    let mut count = 0;

    for chunk in hash.chunks_exact(8) {
        // Convertit 8 bytes en un u64
        let n = u64::from_be_bytes(chunk.try_into().unwrap());

        if n == 0 {
            // 64 bits de zéros
            count += 64;
        } else {
            // utilises l’instruction CPU CLZ :
            count += n.leading_zeros();
            return count;
        }
    }

    count
}
