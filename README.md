## Ceres

The minimal testing environment for ink! contract

## Design

``` text

- - - - - - - - - - - -      - - - - -      - - - -
| ink! contract wasm  | ---> | ceres | ---> | app |
- - - - - - - - - - - -      - - - - -      - - - -

```

Here we use `wasmtime` as the unique choice for the wasm runtime since it is 
the most popular and the most stable choice for running wasm code in rust.

### Calcuating Gas

We use [wasm-utils][0] injecting gas memters for calcuating gas.


## LICENSE

MIT


[0]: https://github.com/paritytech/wasm-utils
