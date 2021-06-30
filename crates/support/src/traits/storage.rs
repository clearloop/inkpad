//! Storage trait
use crate::types::StorageKey;
use ceres_std::Vec;

/// Storage trait
pub trait Storage {
    /// Convert bytes to u32
    fn to_u32(b: &[u8]) -> Option<u32> {
        if b.len() != 4 {
            None
        } else {
            let mut r = [0; 4];
            r.copy_from_slice(b);
            Some(u32::from_ne_bytes(r))
        }
    }

    fn to_storage_key(b: &[u8]) -> Option<StorageKey> {
        if b.len() != 32 {
            None
        } else {
            let mut r = [0; 32];
            r.copy_from_slice(b);
            Some(r)
        }
    }

    /// set K for V
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>>;

    /// set K for V with prefix
    fn prefix_set(&mut self, prefix: &[u8], key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        Self::set(self, [prefix.to_vec(), key].concat(), value)
    }

    /// get V by K
    fn get(&self, key: &[u8]) -> Option<&[u8]>;

    /// get V by K with prefix
    fn prefix_get(&self, prefix: &[u8], key: &[u8]) -> Option<&[u8]> {
        <Self as Storage>::get(self, &[&prefix, key].concat())
    }
}
