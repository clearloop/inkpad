//! common std dependencies
#![cfg(feature = "std")]
pub use std::{
    borrow::ToOwned,
    boxed::Box,
    collections::BTreeMap,
    fmt, format,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};
