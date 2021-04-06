//! Dependencies for adapting std / no_std
mod no_std_dep;
mod std_dep;

#[cfg(not(feature = "std"))]
pub use no_std_dep::*;
#[cfg(feature = "std")]
pub use std_dep::*;
