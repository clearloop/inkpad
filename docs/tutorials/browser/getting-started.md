# Getting Started


### 0. prepare your ink! contract

Please follow the instructions of [prerequisites/ink! contract](/prerequisites/ink-contract.md)
to generate an ink! contract.


### 1. use `@patract/ceres-browser` in your package

```json
{
  dependencies: {
    "@patract/ceres-browser": "^0.1.4"
  }
}
```

### 2. run ink! contract with ceres


```typescript
import { Runtime } from "@ceres/browser";

// this flipper.contract is the output after `0.`
// under /target/ink/flipper.contract
import CONTRACT from "flipper.contract";

(async () => {
    const rt = new Runtime(contract.toString());
    // arguments of call or deploy should be 
    // parity-scale-codec encoded
    rt.deploy("default", []);
    rt.call("default", []);
});
```

The usage of `@patract/ceres-browser` is as same as the rust exports, 
more functions please check [docs.rs/ceres][docs.rs/ceres].


[docs.rs/ceres]: https://docs.rs/ceres-runtime/0.1.0/ceres_runtime/struct.Runtime.html
