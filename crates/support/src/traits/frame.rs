//! frame trait
use crate::types::State;

/// WASM execution frame
pub trait Frame<Memory: 'static + Clone> {
    /// active frame
    fn active(&self) -> Option<[u8; 32]>;

    /// active state
    fn state(&self) -> Option<&State<Memory>>;

    /// active state
    fn state_mut(&mut self) -> Option<&mut State<Memory>>;

    /// Pop frame
    fn pop(&mut self) -> Option<State<Memory>>;

    /// Push frame
    fn push(&mut self, frame: State<Memory>);
}
