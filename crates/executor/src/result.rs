//! Ceres executor result
use crate::trap::Trap;
use ceres_std::{fmt, format, String, Vec};

// #[cfg(not(feature = "std"))]
// use wasmi::Error as E;
// #[cfg(feature = "std")]
// type E = String;

/// Ceres executor errors
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InitMemoryFailed,
    /// Memory outof bounds
    OutOfBounds,
    InitModuleFailed,
    ExecuteFailed(String),
    Trap(Trap),
    CreateWasmtimeConfigFailed,
    GetExternalFailed(String),
    DecodeRuntimeValueFailed,
    OutputBufferTooSmall,
    WrongArugmentLength,
    SetStorageFailed,
    ReturnData {
        flags: u32,
        data: Vec<u8>,
    },
    /// Topics
    TooManyTopics,
    DuplicateTopics,
    TopicValueTooLarge,
    /// Gas
    OutOfGas,
    /// Custom Error
    Custom(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error> {
        f.write_str(&format!("{:?}", &self))?;
        Ok(())
    }
}

/// Ceres executor result
pub type Result<T> = core::result::Result<T, Error>;
