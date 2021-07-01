//! frame trait
use crate::types::State;

/// WASM execution frame
pub trait Frame {
    /// active frame
    fn active(&self) -> Option<[u8; 32]>;

    /// active state
    fn state(&self) -> Option<&State>;

    /// active state
    fn state_mut(&mut self) -> Option<&mut State>;

    /// Pop frame
    fn pop(&mut self) -> Option<State>;

    /// Push frame
    fn push(&mut self, frame: State);
}
