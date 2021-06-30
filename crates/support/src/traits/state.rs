//! Contract State
use crate::traits::Storage;

/// Contract State
pub trait State<Memory>: Storage {
    /// Pop memory
    fn pop_memory(&mut self) -> Option<Memory>;

    /// Push memory
    fn push_memory(&mut self, memory: Memory) -> Option<()>;

    /// Get memory
    fn memory(&self) -> Option<Memory>;

    /// Get memory mut
    fn memory_mut(&mut self) -> Option<&mut Memory>;
}
