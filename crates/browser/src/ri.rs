//! runtime interfaces
use blake2_rfc::blake2b;
use ceres_executor::{
    derive::{ReturnValue, Value},
    Error, Result,
};
use ceres_sandbox::Sandbox;
use ceres_seal::RuntimeInterfaces;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Return type
type Ret = Result<ReturnValue>;

/// Browser interface
pub struct Interface;

impl RuntimeInterfaces for Interface {
    /// Check if enabled
    fn enabled(&self) -> bool {
        true
    }

    /// Println
    fn seal_println(sandbox: &mut Sandbox, args: &[Value]) -> Ret {
        if args.len() != 2 {
            return Err(Error::WrongArugmentLength);
        }

        let data = sandbox.read_sandbox_memory(args[0].into(), args[1].into())?;
        if let Ok(utf8) = core::str::from_utf8(&data) {
            log(utf8);
        }

        Ok(ReturnValue::Unit)
    }

    /// Generate random value
    fn seal_random(sandbox: &mut Sandbox, args: &[Value]) -> Ret {
        if args.len() != 4 {
            return Err(Error::WrongArugmentLength);
        }
        let subject_ptr = args[0].into();
        let subject_len = args[1].into();
        let output_ptr: u32 = args[2].into();
        let output_len: u32 = args[2].into();

        // random
        let random = rand::random::<u8>();
        let mut subject_buf = sandbox
            .read_sandbox_memory(subject_ptr, subject_len)?
            .to_vec();
        subject_buf.push(random);

        let output = blake2b::blake2b(32, &[], &subject_buf);
        sandbox.write_sandbox_output(output_ptr, output_len, output.as_bytes())?;
        Ok(ReturnValue::Unit)
    }

    /// sha2 256
    fn seal_hash_sha2_256(sandbox: &mut Sandbox, args: &[Value]) -> Ret {
        if args.len() != 3 {
            return Err(Error::WrongArugmentLength);
        }
        let input_ptr = args[0].into();
        let input_len = args[1].into();
        let output_ptr = args[2].into();

        // hash
        let mut dest: [u8; 32] = [0; 32];
        let mut hasher = Sha256::new();
        let input = sandbox.read_sandbox_memory(input_ptr, input_len)?;
        hasher.update(&input);
        dest.copy_from_slice(&hasher.finalize());
        sandbox.write_sandbox_memory(output_ptr, dest.as_ref())?;

        // result
        Ok(ReturnValue::Unit)
    }

    /// keccak 256
    fn seal_hash_keccak_256(sandbox: &mut Sandbox, args: &[Value]) -> Ret {
        if args.len() != 3 {
            return Err(Error::WrongArugmentLength);
        }
        let input_ptr = args[0].into();
        let input_len = args[1].into();
        let output_ptr = args[2].into();

        // hash
        let mut dest: [u8; 32] = [0; 32];
        let mut keccak = Keccak::v256();
        let input = sandbox.read_sandbox_memory(input_ptr, input_len)?;
        keccak.update(&input);
        keccak.finalize(&mut dest);
        sandbox.write_sandbox_memory(output_ptr, dest.as_ref())?;

        // result
        Ok(ReturnValue::Unit)
    }

    /// blake2 256
    fn seal_hash_blake2_256(sandbox: &mut Sandbox, args: &[Value]) -> Ret {
        if args.len() != 3 {
            return Err(Error::WrongArugmentLength);
        }
        let input_ptr = args[0].into();
        let input_len = args[1].into();
        let output_ptr = args[2].into();

        // hash
        let mut dest = [0; 32];
        let input = sandbox.read_sandbox_memory(input_ptr, input_len)?;
        dest.copy_from_slice(blake2b::blake2b(32, &[], &input).as_bytes());
        sandbox.write_sandbox_memory(output_ptr, dest.as_ref())?;

        // result
        Ok(ReturnValue::Unit)
    }

    /// blake2 128
    fn seal_hash_blake2_128(sandbox: &mut Sandbox, args: &[Value]) -> Ret {
        if args.len() != 3 {
            return Err(Error::WrongArugmentLength);
        }
        let input_ptr = args[0].into();
        let input_len = args[1].into();
        let output_ptr = args[2].into();

        // hash
        let mut dest = [0; 16];
        let input = sandbox.read_sandbox_memory(input_ptr, input_len)?;
        dest.copy_from_slice(blake2b::blake2b(16, &[], &input).as_bytes());
        sandbox.write_sandbox_memory(output_ptr, dest.as_ref())?;

        // result
        Ok(ReturnValue::Unit)
    }
}
