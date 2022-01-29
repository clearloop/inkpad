# Getting Started


### 0. Template contract

First of all, we need an ink! contract, flipper here.

```
cargo contract new flipper
```


### 1. add `build.rs` to our contract

```rust
// flipper/build.rs
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    Command::new("cargo").args(&["contract", "build"]);
}
```


### 2. add inkpad as `dev-dependencies`

```toml
[dev-dependencies]
inkpad = "^0"
```


### 3. write tests with inkpad

```rust
// /tests/flipper.rs
use inkpad_executor::{Trap, TrapCode};
use inkpad_ri::Instance;
use inkpad_runtime::Runtime;
use parity_scale_codec::Encode;

#[test]
fn test_flipper() {
    let mut rt = Runtime::contract(
        include_bytes!("../contracts/flipper.contract"),
        Some(Instance),
    )
    .expect("Create runtime failed");

    rt.deploy("default", vec![], None).expect("Deploy failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![0])));

    rt.deploy("new", vec![true.encode()], None)
        .expect("Deploy failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![1])));

    rt.call("flip", vec![], None).expect("Call contract failed");
    assert_eq!(rt.call("get", vec![], None), Ok(Some(vec![0])));
}
```

### 4. run tests

```
# flipper/
cargo test
```
