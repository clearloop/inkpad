//! frame trait
use crate::{convert, traits::Storage};

/// WASM execution frame
pub trait Frame: Storage {
    fn frame_prefix(&self) -> &[u8] {
        &[0, 0, 0, 0]
    }

    /// Current id
    fn id(&self) -> Option<u32> {
        convert::to_u32(&self.get(self.frame_prefix())?)
    }

    /// active frame
    fn active(&self) -> Option<&[u8]> {
        self.prefix_get(self.frame_prefix(), self.get(self.frame_prefix())?)
    }

    /// Pop frame
    fn pop_frame(&mut self) -> Option<Vec<u8>> {
        let id = self.id()?;
        if id == 0 {
            return None;
        }

        let last = (id - 1).to_ne_bytes();
        self.set(self.frame_prefix().to_vec(), last.to_vec())?;
        self.prefix_remove(&self.frame_prefix().to_vec(), &id.to_ne_bytes().to_vec())
    }

    /// Push frame
    fn push_frame(&mut self, key: &[u8]) -> Option<Vec<u8>> {
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
