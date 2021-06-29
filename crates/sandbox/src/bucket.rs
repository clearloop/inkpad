//! Sandbox bucket
//!
//! This is for independent contract runtimes
use crate::Sandbox;
use ceres_executor::{derive::SealCall, Error, ExecResult, Memory};
use ceres_std::{Box, Rc};
use ceres_support::traits::{Executor, Storage};
use core::cell::RefCell;

/// Independent contract runtimes
pub struct Bucket {
    state: Box<dyn Storage>,
    memory: Memory,
    executor: Rc<RefCell<dyn Executor<Sandbox, SealCall<Sandbox>, ExecResult, Error>>>,
}
impl Bucket {
    /// New bucket
    pub fn new(
        state: impl Storage + 'static,
        memory: Memory,
        executor: Rc<RefCell<dyn Executor<Sandbox, SealCall<Sandbox>, ExecResult, Error>>>,
    ) -> Self {
        Self {
            state: Box::new(state),
            memory,
            executor,
        }
    }
}
