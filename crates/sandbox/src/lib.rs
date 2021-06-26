#![cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
extern crate bitflags;

use ceres_executor::Memory;
use ceres_std::{vec, Rc, Vec};
use ceres_support::traits::{Executor, Storage};
use core::cell::RefCell;

/// Custom storage key
pub type StorageKey = [u8; 32];

mod chain;
mod contract;
mod ext;
mod flag;
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

use self::{ext::Ext, flag::ExecReturnValue};
pub use self::{flag::ReturnFlags, ri::RuntimeInterfaces, tx::Transaction};
use ceres_executor::{derive::SealCall, Error, ReturnData};

/// The runtime of ink! machine
pub struct Sandbox {
    pub input: Option<Vec<u8>>,
    pub ret: Option<Vec<u8>>,
    pub ext: Ext,
    pub tx: tx::Transaction,
    pub cache: Rc<RefCell<dyn Storage>>,
    pub state: Rc<RefCell<dyn Storage>>,
    memory: Memory,
    pub events: Vec<(Vec<[u8; 32]>, Vec<u8>)>,
    pub ri: Vec<SealCall<Self>>,
    pub executor: Rc<RefCell<dyn Executor<Sandbox, SealCall<Sandbox>, ReturnData, Error>>>,
}

impl Sandbox {
    /// New sandbox
    pub fn new(
        memory: Memory,
        cache: Rc<RefCell<impl Storage + 'static>>,
        state: Rc<RefCell<impl Storage + 'static>>,
        ri: Vec<SealCall<Self>>,
        executor: Rc<
            RefCell<impl Executor<Sandbox, SealCall<Sandbox>, ReturnData, Error> + 'static>,
        >,
    ) -> Sandbox {
        Sandbox {
            input: None,
            ret: None,
            ext: Ext {
                instantiates: vec![],
                restores: vec![],
                rent_allowance: [0; 32],
                terminations: vec![],
                transfers: vec![],
                schedule: Default::default(),
                rent_params: Default::default(),
                gas_meter: Default::default(),
            },
            events: vec![],
            tx: Default::default(),
            cache,
            state,
            memory,
            ri,
            executor,
        }
    }
}
