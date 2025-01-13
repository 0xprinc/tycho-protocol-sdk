use sha2::{Digest, Sha256};
use hex;

pub enum StoreKey {
    Dex,
}

impl StoreKey {
    /// Generates a unique key for a dex using its address by hashing it with SHA-256.
    pub fn get_unique_dex_key(&self, dex_address: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(dex_address.as_bytes());
        let hash = hasher.finalize();
        hex::encode(hash)
    }

    pub fn unique_id(&self) -> String {
        match self {
            StoreKey::Dex => "Dex".to_string(),
        }
    }
}
