//! Cache trait
use crate::traits::{Frame, State};

/// Cache traits
pub trait Cache<Memory>: State<Memory> + Frame {
    /// Active set
    fn active_set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.prefix_set(&self.active()?.to_vec(), key, value)
    }

    /// Active get
    fn active_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.prefix_get(&self.active()?, key)
    }

    /// Enter frame
    fn enter(&mut self, frame: &[u8], memory: Memory) {
        self.push_frame(frame);
        self.push_memory(memory);
    }

    /// Exit frame
    fn exit(&mut self) {
        self.pop_frame();
        self.pop_memory();
    }
}
