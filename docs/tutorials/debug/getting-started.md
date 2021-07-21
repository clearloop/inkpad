# Getting Started

### 0. Compile ink! contract with debug info

For `cargo-contract`, see [3.2](../../prerequisites/cargo-contract.md).


### 1. Introduce `Trap` in ceres


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

`ceres` supports `Trap` both from `wasmi` and `wasmtime`

* [wasmi trap][wasmi]
* [wasmtime trap][wasmtime]


[wasmi]: https://github.com/paritytech/wasmi/blob/a899ebb8d6eeb7a08029735cc03fd54b546cb791/src/lib.rs#L177
[wasmtime]: https://github.com/bytecodealliance/wasmtime/blob/065190f975e7e94650d85d35d3fbea5448a746bd/crates/wasmtime/src/trap.rs#L55


