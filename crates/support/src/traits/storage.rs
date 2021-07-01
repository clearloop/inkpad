//! Storage trait
use ceres_std::Vec;

/// Storage trait
pub trait Storage {
    /// set K for V
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>>;

    /// get V by K
    ///
    /// use `Vec<u8>` as return because some implementation
    /// is hard to return `&[u8]`
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Remove a key
    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>>;
}
