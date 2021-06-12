//! ## Call contracts
//!
//! - deploy callee contracts
//!   - deploy `adder.contract`
//!   - deploy `subber.contract`
//!   - deploy `accumulator.contract`
//!
//! - deploy `delegator.contract`
//!
//! - test `delegator`
//!   - call `get`
//!   - call `change`
//!   - call `switch`
use ceres_runtime::{MemoryStorage, Runtime};

fn test_call_contracts() {
    let shared = MemoryStorage::new();

    // let adder =
    //     Runtime::from_contract_and_storage(include_bytes!("../contracts/adder.contract"), shared);
    // let subber =
    //     Runtime::from_contract_and_storage(include_bytes!("../contracts/subber.contract"), shared);
    // let accumulator = Runtime::from_contract_and_storage(
    //     include_bytes!("../contracts/accumulator.contract"),
    //     shared,
    // );
}
