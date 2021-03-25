//! Custom result
use alloc::vec::Vec;
use snafu::Snafu;
use wasmi::HostError;

/// Ceres Error
#[derive(Snafu, Debug)]
pub enum Error {
    /// Memory out of bounds
    OutOfBounds,
    /// Decoding failed in sandbox
    DecodingFailed,
    /// Output buffer too small
    OutputBufferTooSmall,
    #[snafu(display("flags: {}, data: {:?}", flags, data))]
    ReturnData { flags: u32, data: Vec<u8> },
    /// Wasmi trap
    #[snafu(context(false))]
    Trap { source: wasmi::Trap },
}

impl HostError for Error {}

/// Wrapped result
pub type Result<T> = core::result::Result<T, Error>;
