//! Custom result
use alloc::vec::Vec;
use snafu::Snafu;
use wasmi::HostError;

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
    /// Wasmi trap
    #[snafu(context(false))]
    Trap { source: wasmi::Trap },
    /// Failed to parse wasm module
    ParseWasmModuleFailed,
    /// Failed to parse name section
    ParseWasmNameSectionFailed,
    /// Failed to calcuate memory limit
    CalcuateMemoryLimitFailed,
    /// Failed to alloc memory
    AllocMemoryFailed,
    /// Init ModuleInstance failed
    InitModuleFailed,
    /// Deploy contract failed
    DeployContractFailed,
    /// Call contract failed
    CallContractFailed,
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
}

impl HostError for Error {}

/// Wrapped result
pub type Result<T> = core::result::Result<T, Error>;
