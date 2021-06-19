## @patract/ceres

> Run ink! contract anywhere

An browser implementation of ceres

## Example

```
import * as fs from "fs";
import { Runtime } from "@patract/ceres";

(async () => {
    const contract = fs.readFileSync("../../../contracts/flipper.contract");
    const rt = new Runtime(contract.toString());
    rt.deploy("default", []);
    rt.call("default", []);
})();
```

## LICENSE

Apache-2.0
