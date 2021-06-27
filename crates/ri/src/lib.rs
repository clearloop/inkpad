//! Ceres runtime interfaces ( std )
use blake2_rfc::blake2b;
use ceres_executor::{derive::Value, Error, Result};
use ceres_sandbox::{RuntimeInterfaces, Sandbox};
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};

/// runtime interface instance
pub struct Instance;

impl RuntimeInterfaces for Instance {
    fn seal_println(sandbox: &mut Sandbox, args: &[Value]) -> Result<Value> {
        if args.len() != 2 {
            return Err(Error::WrongArugmentLength);
        }

        let data = sandbox.read_sandbox_memory(args[0].into(), args[1].into())?;
        if let Ok(utf8) = core::str::from_utf8(&data) {
            println!("{}", utf8);
        }

        Ok(Value::F32(0))
    }

    fn seal_random(sandbox: &mut Sandbox, args: &[Value]) -> Result<Value> {
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
        Ok(Value::F32(0))
    }

    fn seal_hash_sha2_256(sandbox: &mut Sandbox, args: &[Value]) -> Result<Value> {
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
        Ok(Value::F32(0))
    }

    fn seal_hash_keccak_256(sandbox: &mut Sandbox, args: &[Value]) -> Result<Value> {
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
        Ok(Value::F32(0))
    }

    fn seal_hash_blake2_256(sandbox: &mut Sandbox, args: &[Value]) -> Result<Value> {
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
        Ok(Value::F32(0))
    }

    fn seal_hash_blake2_128(sandbox: &mut Sandbox, args: &[Value]) -> Result<Value> {
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
        Ok(Value::F32(0))
    }
}
