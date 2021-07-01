/// Share extension
pub trait Ext<Memory, SealCall> {
    fn code(&self, hash: [u8; 32]) -> Option<Vec<u8>>;

    /// Provide seal calls
    fn seal_call(&self) -> SealCall;

    /// Provide memory
    fn memory(&self) -> Memory;
}
