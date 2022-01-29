# Getting Started

### 0. implement your `storage`

The `Storage` trait in `inkpad/crate/support` is the entry of inkpad storage, 
first of all, we need to construct a storage for our implementation.

For example, the storage implementation of `inkpad-cli` is like:

```rust
use inkpad_support::traits::{self, Cache, Frame};

/// A inkpad storage implementation using sled
#[derive(Clone)]
pub struct Storage {
    pub db: Db,
    cache: Tree,
    frame: Vec<Rc<RefCell<State<Memory>>>>,
    state: HostState,
}

impl traits::Storage for Storage {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.cache.insert(key, value).ok()?.map(|v| v.to_vec())
    }

    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.cache.remove(key).ok()?.map(|v| v.to_vec())
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.cache.get(key).ok()?.map(|v| v.to_vec())
    }
}

impl Cache<Memory> for Storage {
    fn frame(&self) -> &Vec<Rc<RefCell<State<Memory>>>> {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Vec<Rc<RefCell<State<Memory>>>> {
        &mut self.frame
    }

    fn memory(&self) -> Option<Memory> {
        Some(self.frame.last()?.borrow().memory.clone())
    }

    /// Flush data
    fn flush(&mut self) -> Option<()> {
        for state in self.frame.iter() {
            let state = state.borrow().clone();
            self.state.insert(state.hash, state.state);
        }

        let mut data = if let Some(state) = self.db.get(PREVIOUS_STATE).ok()? {
            HostState::decode(&mut state.as_ref()).ok()?
        } else {
            BTreeMap::new()
        };

        data.append(&mut self.state.clone());
        self.db.insert(PREVIOUS_STATE, data.encode()).ok()?;
        self.db.flush().ok()?;
        Some(())
    }
}

impl Frame<Memory> for Storage {}
```


### 1. construct your seal calls

we need to construct seal calls for different platforms, for example, `inkpad-cli` use 
system interfaces, `inkpad-browser` use browser interfaces.

For example, the seal calls of `inkpad-browser` is like

```rust
use inkpad_sandbox::{RuntimeInterfaces, Sandbox};

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

```

### 2. Build `Runtime` with your implementations

```rust
/// This Instance has `RuntimeInterfaces` trait bundled
use my_runtime_interfaces::Instance;
use my_storage::Storage;

fn main() {
    let storage = Storage::new();
    let rt = Runtime::from_contract(&source, storage, Some(Instance));
}
```

Here we go, a new implementation of inkpad.
