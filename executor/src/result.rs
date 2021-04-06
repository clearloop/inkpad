//! Ceres executor result
use crate::trap::Trap;

/// Ceres executor errors
pub enum Error {
    InitMemoryFailed,
    MemoryOutOfBonds,
    InitModuleFailed,
    ExecuteFailed,
    Trap(Trap),
}

/// Ceres executor result
pub type Result<T> = core::result::Result<T, Error>;
