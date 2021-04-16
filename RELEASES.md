## ßeta-v0.1

A command-line ceres implementation

* command call
* command deploy
* command list
* command info

## ∂lpha v0.3.1

* Support calling contracts in contracts
* Add transaction config to function call


## ∂lpha v0.3

* Add crate ceres-derive, proc-macros for generating host functions
* Add crate ceres-ri
  * Customized runtime interfaces
* Support all host functions (except chain_extension, call contract )


## ∂lpha v0.2

* add `ceres-executor` - arch for different wasm executor
* add `ceres-std` - std adapter
* add `ceres-seal` - custom host functions
* add `ceres-sandbox` - state for contract execution
* remove `ceres-proxy`
* remove `ceres-gui`


## ∂lpha v0.1

* `ceres-runtime`
  * Decode `metadata.json` from `cargo-contract`
  * run all methods of `flipper.contract` with `no_std`

* `ceres-proxy`
  * A GraphQL has set up for GUI in this version

* `ceres-gui`
  * Here is a GUI template of Ceres using the Design System of Radicle Upstream
