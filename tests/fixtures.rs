use ceres_executor::{Builder, Instance, Memory};
use ceres_ri::Instance as RI;
use ceres_runtime::util;
use ceres_sandbox::Sandbox;
use ceres_seal::pallet_contracts;
use parity_wasm::elements::Module;

/// Get a fixture and compile to instance
fn fixture(name: &str) -> Instance<Sandbox> {
    let fixture_path = ["fixtures/", name, ".wat"].concat();
    let wasm = wat::parse_file(fixture_path).expect("Parse fixture failed");

    // Assemble wasm module
    let el = Module::from_bytes(&wasm).expect("Compile wasm module failed");

    // pack sandbox
    let limit = util::scan_imports(&el).expect("Scan imports failed");
    let mem = Memory::new(limit.0, limit.1).expect("New memory failed");
    let mut sandbox = Sandbox::new(mem, Default::default());

    // Generate Instance
    let builder = Builder::new().add_host_parcels(pallet_contracts(RI));
    Instance::new(&wasm, &builder, &mut sandbox).expect("New instance failed")
}
