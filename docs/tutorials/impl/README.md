# write your implementation of inkpad

inkpad supports both `wasmi` and `wasmtime` as wasm executor, which means 
you can run ink! contract with inkpad anywhere.


## When should I use inkpad with `wsmtime` feature?

`wasmtime` is the default and recommanded executor of inkpad, fast and
full features.


## When should I use inkpad with `wasmi` feature?

If you have requirements running inkpad in browser or some IoT devices,
you don't want any `std` dependencies of rust, you need to use inkpad
with `wasmi` feature.


## Why customized implementation?

inkpad supports the seal functions provided by `pallet-contracts` of 
substrate by default, so if you want to use inkpad in some special 
devices, you need to re-implement the seal functions yourselves.
