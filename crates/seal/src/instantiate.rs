//! Instantiate interface
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{
    derive::{ReturnValue, Value},
    Error, Result,
};
use ceres_sandbox::Sandbox;
use parity_scale_codec::Encode;

/// Instantiate a contract with the specified code hash.
///
/// This function creates an account and executes the constructor defined in the code specified
/// by the code hash. The address of this new account is copied to `address_ptr` and its length
/// to `address_len_ptr`. The constructors output buffer is copied to `output_ptr` and its
/// length to `output_len_ptr`. The copy of the output buffer and address can be skipped by
/// supplying the sentinel value of `u32::max_value()` to `output_ptr` or `address_ptr`.
///
/// After running the constructor it is verified that the contract account holds at
/// least the subsistence threshold. If that is not the case the instantiation fails and
/// the contract is not created.
///
/// # Parameters
///
/// - code_hash_ptr: a pointer to the buffer that contains the initializer code.
/// - code_hash_len: length of the initializer code buffer.
/// - gas: how much gas to devote to the execution of the initializer code.
/// - value_ptr: a pointer to the buffer with value, how much value to send.
///   Should be decodable as a `T::Balance`. Traps otherwise.
/// - value_len: length of the value buffer.
/// - input_data_ptr: a pointer to a buffer to be used as input data to the initializer code.
/// - input_data_len: length of the input data buffer.
/// - address_ptr: a pointer where the new account's address is copied to.
/// - address_len_ptr: in-out pointer to where the length of the buffer is read from
///		and the actual length is written to.
/// - output_ptr: a pointer where the output buffer is copied to.
/// - output_len_ptr: in-out pointer to where the length of the buffer is read from
///   and the actual length is written to.
/// - salt_ptr: Pointer to raw bytes used for address derivation. See `fn contract_address`.
/// - salt_len: length in bytes of the supplied salt.
///
/// # Errors
///
/// Please consult the `ReturnCode` enum declaration for more information on those
/// errors. Here we only note things specific to this function.
///
/// An error means that the account wasn't created and no address or output buffer
/// is returned unless stated otherwise.
///
/// `ReturnCode::CalleeReverted`: Output buffer is returned.
/// `ReturnCode::CalleeTrapped`
/// `ReturnCode::BelowSubsistenceThreshold`
/// `ReturnCode::TransferFailed`
/// `ReturnCode::NewContractNotFunded`
/// `ReturnCode::CodeNotFound`
#[host(seal0)]
pub fn seal_instantiate(
    code_hash_ptr: u32,
    code_hash_len: u32,
    _gas: u64,
    value_ptr: u32,
    value_len: u32,
    input_data_ptr: u32,
    input_data_len: u32,
    address_ptr: u32,
    address_len_ptr: u32,
    output_ptr: u32,
    output_len_ptr: u32,
    salt_ptr: u32,
    salt_len: u32,
) -> Result<ReturnValue> {
    let code_hash: [u8; 32] = sandbox.read_sandbox_memory_as(code_hash_ptr, code_hash_len)?;
    let value: u64 = sandbox.read_sandbox_memory_as(value_ptr, value_len)?;
    let input_data = sandbox.read_sandbox_memory(input_data_ptr, input_data_len)?;
    let salt = sandbox.read_sandbox_memory(salt_ptr, salt_len)?;
    let (address, output, _) =
        sandbox.instantiate(code_hash, value, &mut Default::default(), input_data, &salt)?;

    if !output.flags.contains(ceres_sandbox::ReturnFlags::REVERT) {
        sandbox.write_sandbox_output(address_ptr, address_len_ptr, &address.encode())?;
    }
    sandbox.write_sandbox_output(output_ptr, output_len_ptr, &output.data)?;

    Ok(ReturnValue::Unit)
}

/// Make a call to another contract.
///
/// The callees output buffer is copied to `output_ptr` and its length to `output_len_ptr`.
/// The copy of the output buffer can be skipped by supplying the sentinel value
/// of `u32::max_value()` to `output_ptr`.
///
/// # Parameters
///
/// - callee_ptr: a pointer to the address of the callee contract.
///   Should be decodable as an `T::AccountId`. Traps otherwise.
/// - callee_len: length of the address buffer.
/// - gas: how much gas to devote to the execution.
/// - value_ptr: a pointer to the buffer with value, how much value to send.
///   Should be decodable as a `T::Balance`. Traps otherwise.
/// - value_len: length of the value buffer.
/// - input_data_ptr: a pointer to a buffer to be used as input data to the callee.
/// - input_data_len: length of the input data buffer.
/// - output_ptr: a pointer where the output buffer is copied to.
/// - output_len_ptr: in-out pointer to where the length of the buffer is read from
///   and the actual length is written to.
///
/// # Errors
///
/// An error means that the call wasn't successful output buffer is returned unless
/// stated otherwise.
///
/// `ReturnCode::CalleeReverted`: Output buffer is returned.
/// `ReturnCode::CalleeTrapped`
/// `ReturnCode::BelowSubsistenceThreshold`
/// `ReturnCode::TransferFailed`
/// `ReturnCode::NotCallable`
#[host(seal0)]
pub fn seal_call(
    callee_ptr: u32,
    callee_len: u32,
    _gas: u64,
    value_ptr: u32,
    value_len: u32,
    input_data_ptr: u32,
    input_data_len: u32,
    output_ptr: u32,
    output_len_ptr: u32,
) -> Result<ReturnValue> {
    let callee: [u8; 32] = sandbox.read_sandbox_memory_as(callee_ptr, callee_len)?;
    let value: u64 = sandbox.read_sandbox_memory_as(value_ptr, value_len)?;
    let input_data = sandbox.read_sandbox_memory(input_data_ptr, input_data_len)?;
    let output = sandbox.call(callee, value, input_data)?;
    sandbox.write_sandbox_output(output_ptr, output_len_ptr, &output.data)?;

    Ok(ReturnValue::Unit)
}
