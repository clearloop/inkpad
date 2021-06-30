#![cfg_attr(not(feature = "std"), no_std)]
use self::ext::Ext;
use ceres_executor::derive::SealCall;
use ceres_executor::Memory;
use ceres_std::{vec, Rc, Vec};
use ceres_support::traits::Cache;
use core::cell::RefCell;

/// Custom storage key
pub type StorageKey = [u8; 32];

mod chain;
mod contract;
mod ext;
mod instantiate;
mod memory;
mod restore;
mod ri;
mod schedule;
mod storage;
mod termination;
mod transfer;
mod tx;
mod util;

pub use self::{ri::RuntimeInterfaces, tx::Transaction};

/// The runtime of ink! machine
pub struct Sandbox {
    pub input: Option<Vec<u8>>,
    pub ret: Option<Vec<u8>>,
    pub ext: Ext,
    pub tx: tx::Transaction,
    pub cache: Rc<RefCell<dyn Cache<Memory>>>,
    pub events: Vec<(Vec<[u8; 32]>, Vec<u8>)>,
    pub ri: Vec<SealCall<Self>>,
}

impl Sandbox {
    /// New sandbox
    pub fn new(
        cache: Rc<RefCell<impl Cache<Memory> + 'static>>,
        ri: Vec<SealCall<Self>>,
    ) -> Sandbox {
        Sandbox {
            input: None,
            ret: None,
            ext: Default::default(),
            events: vec![],
            tx: Default::default(),
            cache,
            ri,
        }
    }
}
