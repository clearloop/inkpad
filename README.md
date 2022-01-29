<div align="center">
<h1 align="center">
Inkpad
</h1>

[![crate][a1]][a2] [![docs][c1]][c2] [![downloads][d1]][d2] [![LICENSE][e1]][e2] 

[a1]: https://img.shields.io/crates/v/inkpad-runtime.svg
[a2]: https://crates.io/crates/inkpad-runtime
[c1]: https://img.shields.io/badge/current-docs-brightgreen.svg
[c2]: https://docs.rs/inkpad-runtime
[d1]: https://img.shields.io/crates/d/inkpad-runtime.svg
[d2]: https://crates.io/crates/inkpad-runtime
[e1]: https://img.shields.io/crates/l/inkpad-runtime.svg
[e2]: https://choosealicense.com/licenses/apache-2.0/

 <h3>
    <a href="https://clearloop.github.io/inkpad/">Guide</a>
    <span> | </span>
    <a href="./README_ZH.md">Chinese</a>
</h3>

</div>

## Play with it

The Inkpad CLI can be installed with:

```
cargo install inkpad-cli
```

We can use the `inkpad` binary to explore the usages of inkpad library.

```
inkpad 0.1.0
Inkpad command tool

USAGE:
    inkpad [*.contract | name | code-hash] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <*.contract | name | code-hash>    If empty, inkpad will load the last contract which has been executed

SUBCOMMANDS:
    call      Calls a call method
    deploy    Calls a deploy method
    help      Prints this message or the help of the given subcommand(s)
    info      Prints info of *.contract
    list      Lists all contracts
```

## Features

* **lightweight**. Inkpad is a standalone ink! contract environment that scales with your needs, 
Inkpad can be embedded into almost any application.
* **debugging**. Inkpad built with the traps handlers of both `wasmi` and `wasmtime`, it supports
catching the traps of the wasm binary of ink! contracts
* **tests**. With Inkpad, you can write tests of ink! contract with full-features of the chain api
* **customized**. The runtime of inkpad works with various of rust `trait` that we can configure it 
blazing flexible.


## Platform Support

You can use Inkpad from a variety of different platforms:

* Rust - the [inkpad-runtime][inkpad-runtime] crate
* Browser - the [inkpad-browser][inkpad-browser] crate
* Command Line - the [inkpad-cli][inkpad-cli] crate

## Example

```rust
// test flipper.contract
use inkpad_ri::Instance;
use inkpad_runtime::Runtime;
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

Inkpad extracted from `sp-sandbox` and `pallet-contracts` of substrate
with both `wasmi` and `wasmtime` features, plus trap handlers for the
two executors.

It abstracts the chain environment which ink! contracts requires so
that we can embed inkpad almost any applications to run ink! contracts.


---

Run ink! contract anywhere!


[inkpad-runtime]: https://github.com/patractlabs/inkpad/tree/master/crates/runtime
[inkpad-browser]: https://github.com/patractlabs/inkpad/tree/master/browser
[inkpad-cli]: https://github.com/patractlabs/inkpad/tree/master/cli
