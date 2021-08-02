# `patractlabs/cargo-contract`

For compiling ink! contract with debug info, we can use `patractlabs/cargo-contract`
along with `ceres`.

## 0. Installation

```
cargo install --git https://github.com/patractlabs/cargo-contract.git --branch tag-v0.12.1 --force
```

## 1. Compile with debug info

```
 𝝺 cargo contract build -h
cargo-contract-build 0.10.0
Executes build of the smart-contract which produces a wasm binary that is ready for deploying.

It does so by invoking `cargo build` and then post processing the final binary.

USAGE:
    cargo contract build [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Enable debug info in the wasm bundle
    -h, --help       Prints help information
        --quiet      No output printed to stdout
    -V, --version    Prints version information
        --verbose    Use verbose output

OPTIONS:
        --generate <all | code-only>       Which build artifacts to generate. [default: all]
        --manifest-path <manifest-path>    Path to the Cargo.toml of the contract to build
    -Z, --unstable-options <options>...    Use the original manifest (Cargo.toml), do not modify for build optimizations
```

See the `--debug` flag, run `cargo build -d` in the contract path will embed
debug info in the wasm of our contract.
