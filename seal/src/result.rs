//! Ceres host function result
use ceres_std::Vec;

/// Ceres host function errors
#[derive(Debug)]
pub enum Error {
    DecodeRuntimeValueFailed,
    WrongArugmentLength,
    OutOfBounds,
    Sandbox(ceres_sandbox::Error),
    ReturnData { flags: u32, data: Vec<u8> },
}

impl From<ceres_sandbox::Error> for Error {
    fn from(e: ceres_sandbox::Error) -> Error {
        Error::Sandbox(e)
    }
}

/// Ceres host function result
pub type Result<T> = core::result::Result<T, Error>;
