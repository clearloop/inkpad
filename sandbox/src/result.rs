//! Ceres Sandbox Result

/// Ceres sandbox error
pub enum Error {
    DecodeRuntimeValueFailed,
    OutputBufferTooSmall,
    OutOfBounds,
}

/// Ceres Sandbox Result
pub type Result<T> = core::result::Result<T, Error>;
