//! Custom result
use ceres_std::{String, Vec};
use snafu::Snafu;

/// Ceres Error
#[derive(Snafu, Debug)]
pub enum Error {
    /// Memory out of bounds
    OutOfBounds,
    /// Decoding data failed in sandbox
    DecodeRuntimeValueFailed,
    /// Output buffer too small
    OutputBufferTooSmall,
    #[snafu(display("flags: {}, data: {:?}", flags, data))]
    ReturnData { flags: u32, data: Vec<u8> },
    /// Failed to parse wasm module
    ParseWasmModuleFailed,
    /// Failed to parse name section
    #[snafu(display("Failed to parse name section {}", error))]
    ParseNameSectionFailed { error: String },
    /// Failed to calcuate memory limit
    CalcuateMemoryLimitFailed,
    /// Failed to alloc memory
    AllocMemoryFailed,
    #[snafu(display("Serialize failed {}", error))]
    SerializeFailed {
        error: parity_wasm::SerializationError,
    },
    /// Init ModuleInstance failed
    #[snafu(display("Init module failed {}", error))]
    InitModuleFailed { error: ceres_executor::Error },
    /// Deploy contract failed
    #[snafu(display("Deploy contract failed {}", error))]
    DeployContractFailed { error: ceres_executor::Error },
    #[snafu(display("Call contract failed {}", error))]
    CallContractFailed { error: ceres_executor::Error },
    /// Decode selector failed
    DecodeSelectorFailed,
    /// Decode contract failed
    DecodeContractFailed,
    /// The length of arguments is not correct
    InvalidArgumentLength,
    /// Parse args failed
    ParseArgumentFailed,
    #[snafu(display("Could not find method {}", name))]
    GetMethodFailed { name: String },
    /// Could not set Storage
    CouldNotSetStorage,
    /// Get Storage failed
    GetStorageFailed,
    /// Invalid code hash
    InvalidCodeHash,
    #[snafu(display("{}", err))]
    Custom { err: &'static str },
    /// Insert Contract failed
    InsertContractFailed,
    /// Get Contract failed
    GetContractFailed,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::SerializeFailed { error: _ } => false,
            _ => self.eq(other),
        }
    }
}

impl Eq for Error {}

/// Wrapped result
pub type Result<T> = core::result::Result<T, Error>;
