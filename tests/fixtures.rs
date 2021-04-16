use ceres_executor::{Builder, Instance, Memory};
use ceres_ri::Instance as RI;
use ceres_runtime::util;
use ceres_sandbox::Sandbox;
use ceres_seal::pallet_contracts;
use parity_wasm::elements::Module;

const ALICE: &str = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
// const BOB: &str = "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48";

/// Get a fixture and compile to instance
fn fixture(name: &str) -> (Result<Instance<Sandbox>, ceres_executor::Error>, Sandbox) {
    let fixture_path = ["fixtures/", name, ".wat"].concat();
    let wasm = wat::parse_file(fixture_path).expect("Parse fixture failed");

    // Assemble wasm module
    let el = Module::from_bytes(&wasm).expect("Compile wasm module failed");

    // pack sandbox
    let limit = util::scan_imports(&el).expect("Scan imports failed");
    let mem = Memory::new(limit.0, limit.1).expect("New memory failed");
    let mut sandbox = Sandbox::new(mem.clone(), Default::default());
    sandbox.tx.caller = util::parse_code_hash(&ALICE).expect("Parse addr failed");

    // Generate Instance
    let mut builder = Builder::new().add_host_parcels(pallet_contracts(RI));
    builder.add_memory("env", "memory", mem);
    (Instance::new(&wasm, &builder, &mut sandbox), sandbox)
}

#[test]
fn instantiate_and_call_deposit_event() {
    let (r, s) = fixture("return_from_start_fn");
    if let Err(_) = r {
        assert_eq!(s.ret, Some(vec![1, 2, 3, 4]));
    } else {
        panic!("Return data failed");
    }

    assert_eq!(s.events, vec![(vec![], vec![1, 2, 3, 4])]);
}
