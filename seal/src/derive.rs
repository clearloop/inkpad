//! Derive types && traits
use ceres_executor::{
    derive::{HostParcel, ReturnValue, Value},
    Result,
};
use ceres_sandbox::Sandbox;

/// Custom return code for wasm functions
#[repr(u32)]
pub enum ReturnCode {
    /// API call successful.
    Success = 0,
    /// The called function trapped and has its state changes reverted.
    /// In this case no output buffer is returned.
    CalleeTrapped = 1,
    /// The called function ran to completion but decided to revert its state.
    /// An output buffer is returned when one was supplied.
    CalleeReverted = 2,
    /// The passed key does not exist in storage.
    KeyNotFound = 3,
    /// Transfer failed because it would have brought the sender's total balance below the
    /// subsistence threshold.
    BelowSubsistenceThreshold = 4,
    /// Transfer failed for other reasons. Most probably reserved or locked balance of the
    /// sender prevents the transfer.
    TransferFailed = 5,
    /// The newly created contract is below the subsistence threshold after executing
    /// its constructor.
    NewContractNotFunded = 6,
    /// No code could be found at the supplied code hash.
    CodeNotFound = 7,
    /// The contract that was called is either no contract at all (a plain account)
    /// or is a tombstone.
    NotCallable = 8,
}

/// Host function trait
pub trait Host: Sized {
    /// Host function module
    fn module() -> &'static str;

    /// Host function name
    fn name() -> &'static str;

    /// Wrap host function
    fn wrap(sandbox: &mut Sandbox, args: &[Value]) -> Result<ReturnValue>;

    /// Pack instance
    fn pack() -> HostParcel<&'static str, &'static str, Sandbox> {
        (
            <Self as Host>::module(),
            <Self as Host>::name(),
            <Self as Host>::wrap,
        )
    }
}
