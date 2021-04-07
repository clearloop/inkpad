//! Ceres Sandbox Result
use ceres_std::Vec;

/// Ceres sandbox error
#[derive(Debug)]
pub enum Error {
    DecodeRuntimeValueFailed,
    OutputBufferTooSmall,
    OutOfBounds,
}

/// Ceres Sandbox Result
pub type Result<T> = core::result::Result<T, Error>;
