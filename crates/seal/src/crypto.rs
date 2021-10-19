//! crypto fns
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{derive::Value, Error, Result};
use ceres_sandbox::Sandbox;

// Recovers the ECDSA public key from the given message hash and signature.
//
// Writes the public key into the given output buffer.
// Assumes the secp256k1 curve.
//
// # Parameters
//
// - `signature_ptr`: the pointer into the linear memory where the signature
//					  is placed. Should be decodable as a 65 bytes. Traps otherwise.
// - `message_hash_ptr`: the pointer into the linear memory where the message
// 						 hash is placed. Should be decodable as a 32 bytes. Traps otherwise.
// - `output_ptr`: the pointer into the linear memory where the output
//                 data is placed. The buffer should be 33 bytes. Traps otherwise.
// 				   The function will write the result directly into this buffer.
//
// # Errors
//
// `ReturnCode::EcdsaRecoverFailed`
#[host(__unstable__)]
pub fn seal_ecdsa_recover(
    signature_ptr: u32,
    message_hash_ptr: u32,
    output_ptr: u32,
) -> Result<Option<Value>> {
    let mut signature: [u8; 65] = [0; 65];
    sandbox.read_sandbox_memory_into_buf(signature_ptr, &mut signature)?;
    let mut message_hash: [u8; 32] = [0; 32];
    sandbox.read_sandbox_memory_into_buf(message_hash_ptr, &mut message_hash)?;

    let result = sandbox.ecdsa_recover(&signature, &message_hash);

    match result {
        Ok(pub_key) => {
            // Write the recovered compressed ecdsa public key back into the sandboxed output
            // buffer.
            sandbox.write_sandbox_memory(output_ptr, pub_key.as_ref())?;

            Ok(None)
        }
        Err(_) => Ok(Some(Value::I32(11))),
    }
}
