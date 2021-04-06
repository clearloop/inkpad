//! Ceres executor result
use crate::trap::Trap;

/// Ceres executor errors
#[derive(Debug)]
pub enum Error {
    InitMemoryFailed,
    MemoryOutOfBonds,
    InitModuleFailed,
    ExecuteFailed,
    Trap(Trap),
    CreateWasmtimeConfigFailed,
    GetExternalFailed,
}

/// Ceres executor result
pub type Result<T> = core::result::Result<T, Error>;
