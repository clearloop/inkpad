# Ceres

> Run ink! contract anywhere

A browser implementation of ceres


## Example

```typescript
import("@patract/ceres-browser").then((wasm) => {
    const contract = "<hex-encode-of-*.contract>";
    const runtime = new wasm.Runtime(contract);
    
    // deploy contract
    runtime.deploy(
        // method
        "default", 
        // arguments
        JSON.stringify([]),
        // transaction?
        //
        // interface ITransaction {
        //     address: string;
        //     balance: number;
        //     caller: string;
        //     minimum_balance: number;
        //     now: string;
        //     value_transferred: number;
        // }
        JSON.stringify({}),
    );
    
    // call contract
    const result = runtime.call(
      // method
      "flip",
      // arguments
      JSON.stringify([]),
      // transaction
      JSON.stringify({}),
    );
}).catch(console.error);
```

## LICENSE

Apache-2.0

