# Getting Started

### 0. Compile ink! contract with debug info

For `cargo-contract`, see [3.2](../../prerequisites/cargo-contract.md).


### 1. Introduce `Trap` in inkpad


```rust
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
    HostError(Box<Error>),

    // Unknown Error
    Unknown,

    // Termination
    Termination,

    // Restoration
    Restoration,
}

```

`inkpad` supports `Trap` both from `wasmi` and `wasmtime`

* [wasmi trap][wasmi]
* [wasmtime trap][wasmtime]


[wasmi]: https://github.com/paritytech/wasmi/blob/a899ebb8d6eeb7a08029735cc03fd54b546cb791/src/lib.rs#L177
[wasmtime]: https://github.com/bytecodealliance/wasmtime/blob/065190f975e7e94650d85d35d3fbea5448a746bd/crates/wasmtime/src/trap.rs#L55


### 2. Embed `panic` in contract

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod flipper_trap {
    #[ink(storage)]
    pub struct FlipperTrap {
        value: bool,
    }

    impl FlipperTrap {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            panic!("trap here");
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}

```

### 3. Catch the trap in contract

```rust
#[test]
fn test_flipper_trap() {
    // Assume we compile the contract in `2.` to `flipper_trap.contract`
    let mut rt = Runtime::contract(
        include_bytes!("flipper_trap.contract"),
        Some(Instance),
    )
    .expect("Create runtime failed");

    rt.deploy("default", vec![], None).expect("Deploy failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![0])));

    if let Some(inkpad_runtime::Error::CallContractFailed {
        error: inkpad_executor::Error::Trap(Trap { code, .. }),
    }) = rt.call("flip", vec![], None).err()
    {
        assert_eq!(code, TrapCode::UnreachableCodeReached);
    } else {
        panic!("Call flipper_trap with unexpected error");
    }
}

```
