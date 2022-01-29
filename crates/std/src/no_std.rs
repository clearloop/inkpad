//! no std dependencies
#![cfg(not(feature = "std"))]
pub use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::BTreeMap,
    fmt, format,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};
