# Getting Started


### 0. prepare your ink! contract

Please follow the instructions of [prerequisites/ink! contract](/prerequisites/ink-contract.md)
to generate an ink! contract.


### 1. use `@patract/inkpad-browser` in your package

```json
{
  dependencies: {
    "@patract/inkpad-browser": "^0.1.4"
  }
}
```

### 2. run ink! contract with inkpad


```typescript
import { Runtime } from "@inkpad/browser";

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

The usage of `@patract/inkpad-browser` is as same as the rust exports, 
more functions please check [docs.rs/inkpad][docs.rs/inkpad].


[docs.rs/inkpad]: https://docs.rs/inkpad-runtime/0.1.0/inkpad_runtime/struct.Runtime.html
