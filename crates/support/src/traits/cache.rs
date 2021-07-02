//! Cache trait
use crate::traits::{Frame, Storage};

/// Cache traits
pub trait Cache<Memory: 'static + Clone>: Frame<Memory> + Storage {}
