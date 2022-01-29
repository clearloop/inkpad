//! std runtime interfaces
use crate::Sandbox;
use inkpad_executor::{derive::Value, Result};
use inkpad_std::{vec, Vec};

type ParcelResult = Result<Option<Value>>;

/// std runtime interfaces
pub trait RuntimeInterfaces: Sized {
    /// Println
    fn seal_println(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// Generate random value
    fn seal_random(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// sha2 256
    fn seal_hash_sha2_256(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// keccak 256
    fn seal_hash_keccak_256(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// blake2 256
    fn seal_hash_blake2_256(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// blake2 128
    fn seal_hash_blake2_128(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// pack functions
    fn pack(&self) -> Vec<inkpad_executor::derive::HostCall<&'static str, &'static str, Sandbox>> {
        vec![
            ("seal0", "seal_debug_message", Self::seal_println),
            ("seal0", "seal_random", Self::seal_random),
            ("seal1", "seal_random", Self::seal_random),
            ("seal0", "seal_hash_blake2_128", Self::seal_hash_blake2_128),
            ("seal0", "seal_hash_blake2_256", Self::seal_hash_blake2_256),
            ("seal0", "seal_hash_keccak_256", Self::seal_hash_keccak_256),
            ("seal0", "seal_hash_sha2_256", Self::seal_hash_sha2_256),
        ]
    }
}
