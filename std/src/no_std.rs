//! no std dependencies
#![cfg(not(feature = "std"))]
pub use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    fmt, format,
    prelude::v1::Box,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};
