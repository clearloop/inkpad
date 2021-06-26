//! support traits
use crate::types::StorageKey;
use ceres_std::{Rc, Vec};
use core::cell::RefCell;

/// Custom storage
pub trait Storage {
    /// Get bytes by StorageKey
    fn get(&self, key: StorageKey) -> Option<Vec<u8>>;

    /// Set bytes by StorageKey
    fn set(&mut self, key: StorageKey, value: Vec<u8>) -> Option<StorageKey>;
}

/// Cache with executing feature
pub trait Executor {
    /// new executor
    fn new(cache: Rc<RefCell<impl Storage + 'static>>) -> Self;

    /// Call methods from a contract
    fn call(&self, code_hash: StorageKey, method: &str, data: Vec<u8>) -> Option<Vec<u8>>;
}
