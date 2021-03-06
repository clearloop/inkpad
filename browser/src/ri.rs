//! runtime interfaces
use crate::result::err_check;
use blake2_rfc::blake2b;
use inkpad_executor::{derive::Value, Error, Result};
use inkpad_sandbox::{RuntimeInterfaces, Sandbox};
use getrandom::getrandom;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

/// Return type
type Ret = Result<Option<Value>>;

/// Browser interface
pub struct Interface;

impl RuntimeInterfaces for Interface {
    /// Println
    fn seal_println(sandbox: &mut Sandbox, args: &[Value]) -> Ret {
        if args.len() != 2 {
            return Err(Error::WrongArugmentLength);
        }

        let data = sandbox.read_sandbox_memory(args[0].into(), args[1].into())?;
        if let Ok(utf8) = core::str::from_utf8(&data) {
            log(utf8);
        }

        Ok(None)
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
        let mut dest: [u8; 1] = [0];
        err_check(getrandom(&mut dest));
        let mut subject_buf = sandbox
            .read_sandbox_memory(subject_ptr, subject_len)?
            .to_vec();
        subject_buf.push(dest[0]);

        let output = blake2b::blake2b(32, &[], &subject_buf);
        sandbox.write_sandbox_output(output_ptr, output_len, output.as_bytes())?;
        Ok(None)
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
        Ok(None)
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
        Ok(None)
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
        Ok(None)
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
        Ok(None)
    }
}
