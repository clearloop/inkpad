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
    <a href="https://patractlabs.github.io/inkpad/"> 教程 </a>
    <span> | </span>
    <a href="./README.md"> 英文 </a>
</h3>

</div>

## 尝试 Inkpad

可以通过以下命令安装 Inkpad CLI:

```
cargo install inkpad-cli
```

我们可以通过 `inkpad` 命令来探索 Inkpad 库:

```
inkpad 0.2.0
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

## 特性

* **轻量**。 Inkpad 是一个可以根据你的需求进行拓展的 ink! contract 执行环境,
它可以被嵌入到绝大数 Apps 中。
* **debugging**. Inkpad 包含了 `wasmi` 和 `wasmtime` 的 trap handlers，可以捕捉
ink! contract 执行过程中抛出的 traps。
* **tests**. Inkpad 包含了 ink! contract 执行环境的全部特性，你可以通过 Inkpad 
轻松地测试你的 ink! contract。
* **customized**. Inkpad 的 runtime 通过 Rust `trait` 构建，我们可以十分灵活
地定制自己的 Inkpad Runtime。


## 支持平台

你可以在以下平台使用 Inkpad:

* Rust - 请看 [inkpad-runtime][inkpad-runtime]
* 浏览器 - 请看 [inkpad-browser][inkpad-browser]
* 命令行 - 请看 [inkpad-cli][inkpad-cli]

## 案例

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

## Inkpad 如何运作？

Inkpad 给予 substrate 的 `sp-sandbox` 和 `pallet-contracts` 制作，同时
包含了 `wasmi` 和 `wasmtime` 两种执行器，并且为它们添加了 wasm trap 
handlers。

它模拟了 substrate 链的环境，以至于我们可以将 Inkpad 嵌入到任何 Apps
中使用。

---

Run ink! contract anywhere!


[inkpad-runtime]: https://github.com/patractlabs/inkpad/tree/master/crates/runtime
[inkpad-browser]: https://github.com/patractlabs/inkpad/tree/master/browser
[inkpad-cli]: https://github.com/patractlabs/inkpad/tree/master/cli
