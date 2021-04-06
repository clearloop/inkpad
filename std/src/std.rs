//! common std dependencies
#![cfg(feature = "std")]
pub use std::{
    boxed::Box,
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::{self, Vec},
};
