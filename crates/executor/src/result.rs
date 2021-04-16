//! Ceres executor result
use crate::trap::Trap;
use ceres_std::Vec;

/// Ceres executor errors
#[derive(Debug)]
pub enum Error {
    InitMemoryFailed,
    OutOfBounds,
    InitModuleFailed,
    ExecuteFailed,
    Trap(Trap),
    CreateWasmtimeConfigFailed,
    GetExternalFailed,
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
