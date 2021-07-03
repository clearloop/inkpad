//! Custom result
use ceres_std::{String, Vec};

/// Ceres Error
#[derive(Debug)]
pub enum Error {
    /// Memory out of bounds
    OutOfBounds,
    /// Decoding data failed in sandbox
    DecodeRuntimeValueFailed,
    /// Output buffer too small
    OutputBufferTooSmall,
    ReturnData {
        flags: u32,
        data: Vec<u8>,
    },
    /// Failed to parse wasm module
    ParseWasmModuleFailed,
    /// Failed to parse name section
    ParseNameSectionFailed {
        error: String,
    },
    /// Failed to calcuate memory limit
    CalcuateMemoryLimitFailed,
    /// Failed to alloc memory
    AllocMemoryFailed,
    SerializeFailed {
        error: parity_wasm::SerializationError,
    },
    /// Init ModuleInstance failed
    InitModuleFailed {
        error: ceres_executor::Error,
    },
    /// Deploy contract failed
    DeployContractFailed {
        error: ceres_executor::Error,
    },
    CallContractFailed {
        error: ceres_executor::Error,
    },
    /// Decode selector failed
    DecodeSelectorFailed,
    /// Decode contract failed
    DecodeContractFailed,
    /// The length of arguments is not correct
    InvalidArgumentLength {
        expect: usize,
        input: usize,
    },
    /// Decode argument failed
    DecodeArgumentFailed {
        arg: Vec<u8>,
    },
    GetMethodFailed {
        name: String,
    },
    /// Could not set Storage
    CouldNotSetStorage,
    /// Get Storage failed
    GetStorageFailed,
    /// Invalid code hash
    InvalidCodeHash,
    Custom(&'static str),
    /// Insert Contract failed
    InsertContractFailed,
    /// Get Contract failed
    GetContractFailed,
    /// SerdeError
    SerdeError,
    ExecutorNotInited,
    InitExecutorFailed,
    /// Executor Error
    ExecuteWasmFailed(ceres_executor::Error),
    LoadDataFailed,
    FlushDataFailed,
}

impl From<ceres_executor::Error> for Error {
    fn from(e: ceres_executor::Error) -> Error {
        Error::ExecuteWasmFailed(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Error {
        Error::Custom(e)
    }
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
