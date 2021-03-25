//! POC
//!
//! Test invoking ink! functions in wasmtime
use ceres_runtime::{scan_imports, Metadata, Resolver};
use wasmi::{memory_units::Pages, MemoryInstance, Module, ModuleInstance, RuntimeValue};

#[test]
fn test_invoke() {
    let meta = serde_json::from_str::<Metadata>(&String::from_utf8_lossy(include_bytes!(
        "../flipper.contract"
    )))
    .expect("Could not parse flipper.contract");

    let raw = hex::decode(&meta.source.wasm.as_bytes()[2..])
        .expect("Decode wasm field in contract failed");
    let buf = Module::from_buffer(&raw).expect("Could not parse wasm field in contract");
    let limits = scan_imports(&raw)
        .expect("Could not calcuate memory limits")
        .expect("Could not find memory env");
    let mem = MemoryInstance::alloc(
        Pages(limits.0 as usize),
        limits.1.map(|v| Pages(v as usize)),
    )
    .expect("Alloc memory failed");

    let mut resolver = Resolver::new(mem);
    let module = ModuleInstance::new(&buf, &resolver)
        .expect("Initial wasm moudle instance failed")
        .assert_no_start();
    // println!("{:?}", hex::decode("ed4b9d1b").unwrap());
    // [237, 75, 157, 27]
    // module
    //     .invoke_export("deploy", &[], &mut resolver)
    //     .expect("construct contract failed");
}
