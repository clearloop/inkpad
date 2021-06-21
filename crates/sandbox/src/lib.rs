#![cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
extern crate bitflags;

use ceres_executor::Memory;
use ceres_std::{vec, Rc, Vec};
use ceres_support::traits::Storage;
use core::cell::RefCell;

/// Custom storage key
pub type StorageKey = [u8; 32];

mod chain;
mod contract;
mod instantiate;
mod memory;
mod restore;
mod schedule;
mod storage;
mod termination;
mod transfer;
mod tx;
mod util;

use self::{
    contract::{GasMeter, RentParams},
    schedule::Schedule,
};
use parity_scale_codec::{Decode, Encode};
pub use tx::Transaction;

bitflags! {
    /// Flags used by a contract to customize exit behaviour.
    #[derive(Encode, Decode)]
    pub struct ReturnFlags: u32 {
        /// If this bit is set all changes made by the contract execution are rolled back.
        const REVERT = 0x0000_0001;
    }
}

/// Return flags
pub struct ExecReturnValue {
    pub flags: ReturnFlags,
    pub data: Vec<u8>,
}

/// Extend data
pub struct Ext {
    pub instantiates: Vec<instantiate::InstantiateEntry>,
    pub restores: Vec<restore::RestoreEntry>,
    pub rent_allowance: [u8; 32],
    pub terminations: Vec<termination::TerminationEntry>,
    pub transfers: Vec<transfer::TransferEntry>,
    pub schedule: Schedule,
    pub rent_params: RentParams,
    pub gas_meter: GasMeter,
}

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
}

impl Sandbox {
    /// New sandbox
    pub fn new(
        memory: Memory,
        cache: Rc<RefCell<impl Storage + 'static>>,
        state: Rc<RefCell<impl Storage + 'static>>,
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
        }
    }
}
