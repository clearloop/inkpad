//! Cache trait
use crate::{traits::Storage, types::State};
use ceres_std::Rc;
use core::cell::RefCell;

/// Cache traits
pub trait Cache<Memory: 'static + Clone>: Storage {
    /// Get frame
    fn frame(&self) -> &Vec<Rc<RefCell<State<Memory>>>>;

    /// Get frame mut
    fn frame_mut(&mut self) -> &mut Vec<Rc<RefCell<State<Memory>>>>;

    /// Memory
    fn memory(&self) -> Option<Memory>;
}
