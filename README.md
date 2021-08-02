<div align="center">
<h1 align="center">
Ceres
</h1>

[![crate][a1]][a2] [![docs][c1]][c2] [![downloads][d1]][d2] [![LICENSE][e1]][e2] 

[a1]: https://img.shields.io/crates/v/ceres-runtime.svg
[a2]: https://crates.io/crates/ceres-runtime
[c1]: https://img.shields.io/badge/current-docs-brightgreen.svg
[c2]: https://docs.rs/ceres-runtime
[d1]: https://img.shields.io/crates/d/ceres-runtime.svg
[d2]: https://crates.io/crates/ceres-runtime
[e1]: https://img.shields.io/crates/l/ceres-runtime.svg
[e2]: https://choosealicense.com/licenses/apache-2.0/

[Run ink! contract anywhere!](https://patractlabs.github.io/ceres/)

</div>

## Play with it

The Ceres CLI can be installed with:

```
cargo install ceres-cli
```

We can use the `ceres` binary to explore the usages of ceres library.

```
ceres 0.2.0
Ceres command tool

USAGE:
    ceres [*.contract | name | code-hash] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <*.contract | name | code-hash>    If empty, ceres will load the last contract which has been executed

SUBCOMMANDS:
    call      Calls a call method
    deploy    Calls a deploy method
    help      Prints this message or the help of the given subcommand(s)
    info      Prints info of *.contract
    list      Lists all contracts
```

## Features

* **lightweight**. Ceres is a standalone ink! contract environment that scales with your needs, 
Ceres can be embedded into almost any application.
* **debugging**. Ceres built with the traps handlers of both `wasmi` and `wasmtime`, it supports
catching the traps of the wasm binary of ink! contracts
* **tests**. With Ceres, you can write tests of ink! contract with full-features of the chain api
* **customized**. The runtime of ceres works with various of rust `trait` that we can configure it 
blazing flexible.


## Platform Support

You can use Ceres from a variety of different platforms:

* Rust - the [ceres-runtime][ceres-runtime] crate
* Browser - the [ceres-browser][ceres-browser] crate
* Command Line - the [ceres-cli][ceres-cli] crate

## Example

```rust
// test flipper.contract
use ceres_ri::Instance;
use ceres_runtime::Runtime;
use parity_scale_codec::Encode;

#[test]
fn test_flipper() {
    let mut rt = Runtime::contract(
        include_bytes!("./flipper.contract"),
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

## How it works

Ceres extracted from `sp-sandbox` and `pallet-contracts` of substrate,



[ceres-runtime]: https://github.com/patractlabs/ceres/tree/master/crates/runtime
[ceres-browser]: https://github.com/patractlabs/ceres/tree/master/browser
[ceres-cli]: https://github.com/patractlabs/ceres/tree/master/cli
