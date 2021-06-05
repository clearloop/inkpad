//! wasm traps
use ceres_std::{String, Vec};
// use core::convert::TryFrom;

/// A trap code describing the reason for a trap.
///
/// All trap instructions have an explicit trap code.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrapCode {
    /// The current stack space was exhausted.
    StackOverflow,

    /// An out-of-bounds memory access.
    MemoryOutOfBounds,

    /// A wasm atomic operation was presented with a not-naturally-aligned linear-memory address.
    HeapMisaligned,

    /// An out-of-bounds access to a table.
    TableOutOfBounds,

    /// Indirect call to a null table entry.
    IndirectCallToNull,

    /// Signature mismatch on indirect call.
    BadSignature,

    /// An integer arithmetic operation caused an overflow.
    IntegerOverflow,

    /// An integer division by zero.
    IntegerDivisionByZero,

    /// Failed float-to-int conversion.
    BadConversionToInteger,

    /// Code that was supposed to have been unreachable was reached.
    UnreachableCodeReached,

    /// Execution has potentially run too long and may be interrupted.
    Interrupt,

    /// HostError
    HostError,

    // Unknown Error
    Unknown,

    // Termination
    Termination,

    // Restoration
    Restoration,
}

/// Wasm Trap
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trap {
    /// Trap code
    pub code: TrapCode,
    /// Wasm backtrace (in wasmtime, this includes native backtrace)
    pub trace: Vec<String>,
}

impl From<TrapCode> for Trap {
    fn from(code: TrapCode) -> Trap {
        Trap {
            code,
            trace: Vec::new(),
        }
    }
}

// impl TryFrom<anyhow::Error> for Trap {
//     type Error = crate::Error;
//
//     fn try_from(e: anyhow::Error) -> Result<Trap, Self::Error> {
//         e.downcast::<Error::>
//     }
// }
