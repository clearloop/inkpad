//! no_std deps
#![cfg(not(feature = "std"))]
pub use alloc::{
    collections::BTreeMap,
    prelude::v1::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
