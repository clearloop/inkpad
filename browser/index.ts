import Flipper from "./flipper.json";

(async () => {
    const wasm = await import("@patract/ceres-browser").catch(console.error);
    const { Runtime } = wasm && (await wasm.default);

    // create monitor
    console.log("hello, this is a template of ceres");

    // load contract
    const contract = new Runtime(JSON.stringify(Flipper));
    console.log("...init contract to storage");

    // deploy contract
    (contract as any).deploy("default", "[]", null);
    console.log("...deploy contract");

    // call contract
    const res = contract.call("get", "[]", null);
    console.log(`...call contract...${res}`);
})();
