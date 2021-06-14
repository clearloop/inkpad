# Ceres Derive

[![crate](https://img.shields.io/crates/v/ceres-derive.svg)](https://crates.io/crates/ceres-derive)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/ceres-derive/)
[![downloads](https://img.shields.io/crates/d/ceres-derive.svg)](https://crates.io/crates/ceres-derive)
[![LICENSE](https://img.shields.io/crates/l/ceres-derive.svg)](https://choosealicense.com/licenses/apache-2.0/)

Procedural macros for ceres

## `#[host(module)]`

```rust
#[host(seal0)]
fn seal_input(out_ptr: u32, out_len_ptr: u32) -> Result<ReturnValue>;
```

```rust
struct SealInput;

impl Host for SealInput {
    fn module() -> &'static str {
        "seal0"
    }
    
    fn name() -> &'static str {
        "seal_input"
    }
    
    fn wrap() -> HostFuncType<Sandbox> {
        fn(sandbox: &mut Sandbox, args: &[Value]) -> Result<ReturnValue> {
            if args.len() != 2 {
                return Err(Error::WrongArugmentLength);
            }
            
            let [out_ptr, out_len_ptr] = [args[0].into(), args[1].into()];
            seal_input(sandbox, out_ptr, out_len_ptr)
        }
    }
    
    /// Pack instance
    fn pack() -> (&'static str, &'static str, HostFuncType<Sandbox>) {
        (
            <Self as Host>::module(),
            <Self as Host>::name(),
            <Self as Host>::wrap,
        )
    }
}
```



## LICNESE

MIT
