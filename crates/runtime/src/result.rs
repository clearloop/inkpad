//! Custom result
use ceres_std::{String, Vec};
use core::fmt::{self, Display, Formatter};
use snafu::{Snafu, ErrorCompat};
use parity_wasm::SerializationError;

/// A thin wrapper for `SerializeFailedError`
/// which seems does not support `PartialEq`-trait
#[derive(Debug)]
pub struct SerializeFailedError(SerializationError);
impl Display for SerializeFailedError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ErrorCompat for SerializeFailedError {}

impl From<SerializationError> for SerializeFailedError {
    fn from(e: SerializationError) -> Self {
        Self(e)
    }
}

impl PartialEq for SerializeFailedError {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for SerializeFailedError {}

/// Ceres Error
#[derive(Snafu, Debug, PartialEq, Eq)]
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
        error: SerializeFailedError,
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
    #[snafu(display(
        "/// The length of arguments is not correct, expect {}, input: {}",
        expect,
        input
    ))]
    InvalidArgumentLength { expect: usize, input: usize },
    /// Decode argument failed
    #[snafu(display("Decode argument failed {:?}", arg))]
    DecodeArgumentFailed { arg: Vec<u8> },
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
    /// SerdeError
    SerdeError,
}

/// Wrapped result
pub type Result<T> = core::result::Result<T, Error>;
