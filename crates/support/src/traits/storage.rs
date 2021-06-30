//! Storage trait
use ceres_std::Vec;

/// Storage trait
pub trait Storage {
    /// set K for V
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>>;

    /// set K for V with prefix
    fn prefix_set(&mut self, prefix: &[u8], key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        Self::set(self, [prefix.to_vec(), key].concat(), value)
    }

    /// get V by K
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// get V by K with prefix
    fn prefix_get(&self, prefix: &[u8], key: &[u8]) -> Option<Vec<u8>> {
        <Self as Storage>::get(self, &[prefix, key].concat())
    }

    /// Remove a key
    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>>;

    /// Remove a key with prefix
    fn prefix_remove(&mut self, prefix: &[u8], key: &[u8]) -> Option<Vec<u8>> {
        <Self as Storage>::remove(self, &[prefix, key].concat())
    }
}
