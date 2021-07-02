//! frame trait

/// WASM execution frame
pub trait Frame<Memory: 'static + Clone> {
    /// active frame
    fn active(&self) -> Option<[u8; 32]>;

    /// active set
    fn active_set(&self, key: [u8; 32], value: Vec<u8>) -> Option<Vec<u8>>;

    /// active get
    fn active_get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Push frame
    fn push(&mut self, code_hash: [u8; 32], memory: Memory);

    /// Switch to frame
    fn switch(&mut self, code_hash: [u8; 32]) -> Option<()>;

    /// back to last frame
    fn back(&mut self) -> Option<()>;

    /// back to top frame
    fn top(&mut self) -> Option<()>;

    /// Memory
    fn memory(&self) -> Option<Memory>;
}
