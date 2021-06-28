#![cfg_attr(not(feature = "std"), no_std)]
use self::ext::Ext;
use ceres_executor::Memory;
use ceres_executor::{derive::SealCall, Error, ExecResult};
use ceres_std::{vec, BTreeMap, Rc, Vec};
use ceres_support::traits::{Executor, Storage};
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
mod stack;
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
    pub bucket: Rc<RefCell<BTreeMap<StorageKey, Vec<u8>>>>,
    pub cache: Rc<RefCell<dyn Storage>>,
    pub state: Rc<RefCell<dyn Storage>>,
    pub stack: Vec<StorageKey>,
    memory: Memory,
    pub events: Vec<(Vec<[u8; 32]>, Vec<u8>)>,
    pub ri: Vec<SealCall<Self>>,
    pub executor: Rc<RefCell<dyn Executor<Sandbox, SealCall<Sandbox>, ExecResult, Error>>>,
}

impl Sandbox {
    /// New sandbox
    pub fn new(
        frame: StorageKey,
        memory: Memory,
        cache: Rc<RefCell<impl Storage + 'static>>,
        state: Rc<RefCell<impl Storage + 'static>>,
        ri: Vec<SealCall<Self>>,
        executor: Rc<
            RefCell<impl Executor<Sandbox, SealCall<Sandbox>, ExecResult, Error> + 'static>,
        >,
    ) -> Sandbox {
        Sandbox {
            input: None,
            ret: None,
            ext: Default::default(),
            bucket: Default::default(),
            events: vec![],
            stack: vec![frame],
            tx: Default::default(),
            cache,
            state,
            memory,
            ri,
            executor,
        }
    }
}
