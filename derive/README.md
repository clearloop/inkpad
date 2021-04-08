# Ceres Derive

Procedural macros for ceres

## `#[host(module)]`

```rust
#[host(seal0)]
fn seal_input(sandbox: &mut Sandbox, out_ptr: u32, out_len_ptr: u32) -> Result<ReturnValue>;
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
}
```



## LICNESE

MIT
