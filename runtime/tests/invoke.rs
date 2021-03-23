//! POC
//!
//! Test invoking ink! functions in wasmtime
use wasmtime::{Linker, Module, Store, Val};

struct Runtime {
    memory: u32,
}

#[test]
fn test_invoke() {
    let store = Store::default();
    let mut linker = Linker::new(&store);

    linker
        .func("seal0", "seal_get_storage", || {})
        .expect("")
        .func("seal0", "seal_set_storage", || {})
        .expect("")
        .func("seal0", "seal_value_transferred", || {})
        .expect("")
        .func("seal0", "seal_input", || {})
        .expect("")
        .func("seal0", "seal_return", || {})
        .expect("");
}
