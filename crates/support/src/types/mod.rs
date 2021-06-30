//! support types
mod cache;
mod state;

pub use self::cache::Cache;

/// Custom storage key
pub type StorageKey = [u8; 32];
