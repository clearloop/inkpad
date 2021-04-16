//! Ceres executor result
use crate::trap::Trap;
use ceres_std::Vec;

#[cfg(not(feature = "std"))]
use wasmi::Error as E;
#[cfg(feature = "std")]
type E = String;

/// Ceres executor errors
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InitMemoryFailed,
    OutOfBounds,
    InitModuleFailed(E),
    ExecuteFailed,
    Trap(Trap),
    CreateWasmtimeConfigFailed,
    GetExternalFailed(String),
    DecodeRuntimeValueFailed,
    OutputBufferTooSmall,
    WrongArugmentLength,
    SetStorageFailed,
    ReturnData { flags: u32, data: Vec<u8> },
    // Topics
    TooManyTopics,
    DuplicateTopics,
    TopicValueTooLarge,
    // Gas
    OutOfGas,
    // Custom Error
    Custom(&'static str),
}

/// Ceres executor result
pub type Result<T> = core::result::Result<T, Error>;
