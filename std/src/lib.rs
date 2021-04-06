//! Ceres `std` adaptor
#![cfg_attr(not(feature = "std"), no_std)]
mod no_std;
mod std;

#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub use self::no_std::*;
