pub struct StoreKey;

impl StoreKey {
    pub fn get_unique_dex_key(&self, dex_address: &str) -> Vec<u8> {
        // Implement a unique key generation logic, e.g., hashing the dex address
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(dex_address.as_bytes());
        hasher.finalize().to_vec()
    }
}