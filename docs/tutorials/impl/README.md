# write your implementation of ceres

ceres supports both `wasmi` and `wasmtime` as wasm executor, which means 
you can run ink! contract with ceres anywhere.


## When should I use ceres with `wsmtime` feature?

`wasmtime` is the default and recommanded executor of ceres, fast and
full features.


## When should I use ceres with `wasmi` feature?

If you have requirements running ceres in browser or some IoT devices,
you don't want any `std` dependencies of rust, you need to use ceres
with `wasmi` feature.


## Why customized implementation?

ceres supports the seal functions provided by `pallet-contracts` of 
substrate by default, so if you want to use ceres in some special 
devices, you need to re-implement the seal functions yourselves.
