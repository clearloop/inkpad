# Ceres

> Run ink! contract anywhere

A browser implementation of ceres


## Example

```typescript
import Flipper from "./flipper.json";

(async () => {
    const wasm = await import("@patract/ceres-browser").catch(console.error);
    const { Runtime } = wasm && (await wasm.default);

    // create monitor
    console.log("hello, this is a template of ceres");

    // load contract
    const contract = new Runtime(JSON.stringify(Flipper));
    console.log("...init contract");

    // deploy contract
    (contract as any).deploy("default", "[]");
    console.log("...deploy contract");

    // call contract
    const res = contract.call("get", "[]");
    console.log(`...call contract...${res}`);

    // flip
    contract.call("flip", "[]");
    const flip = contract.call("get", "[]");
    console.log(`...call contract...${flip}`);
})();
```

## LICENSE

Apache-2.0

