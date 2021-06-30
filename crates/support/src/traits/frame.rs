//! frame trait
use crate::{traits::Storage, types::StorageKey};

/// WASM execution frame
pub trait Frame: Storage + Sized {
    fn frame_prefix(&self) -> &[u8] {
        &[0, 0, 0, 0]
    }

    /// Current id
    fn id(&self) -> Option<u32> {
        Self::to_u32(&self.get(self.frame_prefix())?)
    }

    /// active frame
    fn active(&self) -> Option<&[u8]> {
        self.prefix_get(self.frame_prefix(), self.get(self.frame_prefix())?)
    }

    /// Pop frame
    fn pop(&mut self) -> Option<StorageKey> {
        let id = self.id()?;
        if id == 0 {
            return None;
        }

        let last = (id - 1).to_ne_bytes();
        self.set(self.frame_prefix().to_vec(), last.to_vec())?;
        Self::to_storage_key(&self.prefix_set(
            &self.frame_prefix().to_vec(),
            id.to_ne_bytes().to_vec(),
            [0; 32].to_vec(),
        )?)
    }

    /// Push frame
    fn push(&mut self, key: StorageKey) -> Option<Vec<u8>> {
        let id = self.id().unwrap_or_default();
        let next_id = (id + 1).to_ne_bytes();
        self.set(self.frame_prefix().to_vec(), next_id.to_vec());
        self.prefix_set(
            &self.frame_prefix().to_vec(),
            next_id.to_vec(),
            key.to_vec(),
        )
    }
}
