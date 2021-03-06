# Inkpad Runtime

[![crate](https://img.shields.io/crates/v/inkpad-runtime.svg)](https://crates.io/crates/inkpad-runtime)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/inkpad-runtime/)
[![downloads](https://img.shields.io/crates/d/inkpad-runtime.svg)](https://crates.io/crates/inkpad-runtime)
[![LICENSE](https://img.shields.io/crates/l/inkpad-runtime.svg)](https://choosealicense.com/licenses/apache-2.0/)

The wasm executor of ink! contract

This repo provides:

* Parsing *.contract to ABI and wasm source
* A wasm runtime for executing ink! contract
  * Few interfaces
  
## Design

``` text

- - - - - - - - -      - - - - -      - - - - - - -
| ink! contract | ---> | inkpad | ---> |  anywhere |
- - - - - - - - -      - - - - -      - - - - - - -

```

## Example

```rust
use inkpad_runtime::Runtime;

#[test]
fn test_flipper() {
    let mut rt = Runtime::from_contract(include_bytes!("../flipper.contract"))
        .expect("Create runtime failed");

    rt.deploy("default", &[]).expect("Deploy failed");
    assert_eq!(&rt.call("get", &[]).expect("Call contract failed"), &[0]);

    rt.deploy("new", &["true"]).expect("Deploy failed");
    assert_eq!(&rt.call("get", &[]).expect("Call contract failed"), &[1]);

    rt.call("flip", &[]).expect("Call contract failed");
    assert_eq!(&rt.call("get", &[]).expect("Call contract failed"), &[0]);
}
```

## LICENSE

MIT


