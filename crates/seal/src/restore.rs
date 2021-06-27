//! Restore interface
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{derive::Value, Error, Result, TrapCode};
use ceres_sandbox::Sandbox;
use ceres_std::vec;

/// Try to restore the given destination contract sacrificing the caller.
///
/// This function will compute a tombstone hash from the caller's storage and the given code hash
/// and if the hash matches the hash found in the tombstone at the specified address - kill
/// the caller contract and restore the destination contract and set the specified `rent_allowance`.
/// All caller's funds are transferred to the destination.
///
/// The tombstone hash is derived as `hash(code_hash, storage_root_hash)`. In order to match
/// this hash to its own hash the restorer must make its storage equal to the one of the
/// evicted destination contract. In order to allow for additional storage items in the
/// restoring contract a delta can be specified to this function. All keys specified as
/// delta are disregarded when calculating the storage root hash.
///
/// On success, the destination contract is restored. This function is diverging and
/// stops execution even on success.
///
/// - `dest_ptr`, `dest_len` - the pointer and the length of a buffer that encodes `T::AccountId`
///    with the address of the to be restored contract.
/// - `code_hash_ptr`, `code_hash_len` - the pointer and the length of a buffer that encodes
///    a code hash of the to be restored contract.
/// - `rent_allowance_ptr`, `rent_allowance_len` - the pointer and the length of a buffer that
///    encodes the rent allowance that must be set in the case of successful restoration.
/// - `delta_ptr` is the pointer to the start of a buffer that has `delta_count` storage keys
///    laid out sequentially.
///
/// # Traps
///
/// - There is no tombstone at the destination address.
/// - Tombstone hashes do not match.
/// - The calling contract is already present on the call stack.
/// - The supplied code_hash does not exist on-chain.
#[host(seal0)]
pub fn restore(
    dest_ptr: u32,
    dest_len: u32,
    code_hash_ptr: u32,
    code_hash_len: u32,
    rent_allowance_ptr: u32,
    rent_allowance_len: u32,
    delta_ptr: u32,
    delta_count: u32,
) -> Result<Value> {
    let dest: [u8; 32] = sandbox.read_sandbox_memory_as(dest_ptr, dest_len)?;
    let code_hash: [u8; 32] = sandbox.read_sandbox_memory_as(code_hash_ptr, code_hash_len)?;
    let rent_allowance: u64 =
        sandbox.read_sandbox_memory_as(rent_allowance_ptr, rent_allowance_len)?;
    let delta = {
        const KEY_SIZE: usize = 32;
        let mut delta = vec![[0; KEY_SIZE]; delta_count as usize];
        let mut key_ptr = delta_ptr;

        for i in 0..delta_count {
            // Read the delta into the provided buffer
            // This cannot panic because of the loop condition
            sandbox.read_sandbox_memory_into_buf(key_ptr, &mut delta[i as usize])?;

            // Offset key_ptr to the next element.
            key_ptr = key_ptr
                .checked_add(KEY_SIZE as u32)
                .ok_or(Error::OutOfBounds)?;
        }

        delta
    };

    // TODO: set limit code size
    //
    // let max_len = sandbox.schedule.limits.code_size;
    sandbox.restore_to(dest, code_hash, rent_allowance, delta)?;
    Err(Error::Trap(TrapCode::Restoration.into()))
}
