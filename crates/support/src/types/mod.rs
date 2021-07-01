//! support types
mod cache;
mod state;

pub use self::{cache::Cache, state::State};

/// Custom storage key
pub type StorageKey = [u8; 32];
