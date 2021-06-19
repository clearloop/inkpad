/**
 * Tests of running flipper
 */
import * as fs from "fs";
import { Runtime } from "@patract/ceres";

(async () => {
    const contract = fs.readFileSync("../../../contracts/flipper.contract");
    const rt = new Runtime(contract.toString());
    rt.deploy("default", []);
})();
