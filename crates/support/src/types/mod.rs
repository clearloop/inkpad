//! support types
mod cache;
mod metadata;
mod state;

pub use self::{cache::Cache, metadata::Metadata, state::State};

/// Custom storage key
pub type StorageKey = [u8; 32];
