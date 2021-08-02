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

 <h3>
    <a href="https://patractlabs.github.io/ceres/"> 教程 </a>
    <span> | </span>
    <a href="./README.md"> 英文 </a>
</h3>

</div>

## 尝试 Ceres

可以通过以下命令安装 Ceres CLI:

```
cargo install ceres-cli
```

我们可以通过 `ceres` 命令来探索 Ceres 库:

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

## 特性

* **轻量**。 Ceres 是一个可以根据你的需求进行拓展的 ink! contract 执行环境,
它可以被嵌入到绝大数 Apps 中。
* **debugging**. Ceres 包含了 `wasmi` 和 `wasmtime` 的 trap handlers，可以捕捉
ink! contract 执行过程中抛出的 traps。
* **tests**. Ceres 包含了 ink! contract 执行环境的全部特性，你可以通过 Ceres 
轻松地测试你的 ink! contract。
* **customized**. Ceres 的 runtime 通过 Rust `trait` 构建，我们可以十分灵活
地定制自己的 Ceres Runtime。


## 支持平台

你可以在以下平台使用 Ceres:

* Rust - 请看 [ceres-runtime][ceres-runtime]
* 浏览器 - 请看 [ceres-browser][ceres-browser]
* 命令行 - 请看 [ceres-cli][ceres-cli]

## 案例

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

## Ceres 如何运作？

Ceres 给予 substrate 的 `sp-sandbox` 和 `pallet-contracts` 制作，同时
包含了 `wasmi` 和 `wasmtime` 两种执行器，并且为它们添加了 wasm trap 
handlers。

它模拟了 substrate 链的环境，以至于我们可以将 Ceres 嵌入到任何 Apps
中使用。

---

Run ink! contract anywhere!


[ceres-runtime]: https://github.com/patractlabs/ceres/tree/master/crates/runtime
[ceres-browser]: https://github.com/patractlabs/ceres/tree/master/browser
[ceres-cli]: https://github.com/patractlabs/ceres/tree/master/cli
