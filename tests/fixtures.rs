use ceres_executor::{Builder, Instance, Memory};
use ceres_ri::Instance as RI;
use ceres_runtime::util;
use ceres_sandbox::Sandbox;
use ceres_seal::pallet_contracts;
use ceres_std::Rc;
use core::cell::{RefCell, RefMut};
use parity_scale_codec::Encode;
use parity_wasm::elements::Module;

const ALICE: &str = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

/// Get a fixture and compile to instance
fn fixture(
    name: &str,
) -> (
    Result<Instance<Sandbox>, ceres_executor::Error>,
    Rc<RefCell<Sandbox>>,
) {
    let fixture_path = ["fixtures/", name, ".wat"].concat();
    let wasm = wat::parse_file(fixture_path).expect("Parse fixture failed");

    // Assemble wasm module
    let el = Module::from_bytes(&wasm).expect("Compile wasm module failed");

    // pack sandbox
    let limit = util::scan_imports(&el).expect("Scan imports failed");
    let mem = Memory::new(limit.0, limit.1).expect("New memory failed");
    let sandbox = Rc::new(RefCell::new(Sandbox::new(mem.clone(), Default::default())));
    let sandbox_cloned = sandbox.clone();
    let mut bm = sandbox_cloned.borrow_mut();
    bm.tx
        .set_caller(util::parse_code_hash(&ALICE).expect("Parse addr failed"));

    // Generate Instance
    let mut builder = Builder::new().add_host_parcels(pallet_contracts(RI));
    builder.add_memory("env", "memory", mem);
    (Instance::new(&wasm, &builder, &mut bm), sandbox)
}

/// Shortcut of fxitures
fn f(name: &str, f: fn(i: Instance<Sandbox>, s: RefMut<Sandbox>)) {
    let (r, s) = fixture(name);

    // set initialized balance
    let mut bm = s.borrow_mut();
    bm.tx.set_balance(1_000_000);

    // deploy by default
    let mut instance = r.expect("Instantiate wasm module failed");
    instance
        .invoke("deploy", &[], &mut bm)
        .expect("Deploy failed");

    // closures
    f(instance, bm)
}

#[test]
fn instantiate_and_call_deposit_event() {
    let (r, s) = fixture("return_from_start_fn");
    if let Err(_) = r {
        assert_eq!(s.borrow().ret, Some(vec![1, 2, 3, 4]));
    } else {
        panic!("Return data failed");
    }

    assert_eq!(s.borrow().events, vec![(vec![], vec![1, 2, 3, 4])]);
}

#[test]
fn deposit_event_max_value_limit() {
    f("event_size", |mut i, mut bm| {
        bm.input = Some(bm.max_value_size().encode());
        i.invoke("call", &[], &mut bm).expect("Invoke failed");

        bm.input = Some((bm.max_value_size() + 1).encode());
        assert!(i.invoke("call", &[], &mut bm).is_ok());
    });
}

// #[test]
// fn call_return_code() {}

// #[test]
// fn instantiate_return_code() {}
