#![cfg_attr(not(feature = "std"), no_std)]
use ceres_executor::Memory;
use ceres_std::{vec, BTreeMap, Vec};

/// Custom storage key
pub type StorageKey = [u8; 32];

mod chain;
mod contract;
mod hash;
mod instantiate;
mod memory;
mod restore;
mod schedule;
mod storage;
mod termination;
mod transfer;
mod util;

use self::{
    contract::{GasMeter, RentParams},
    schedule::Schedule,
};

/// Return flags
pub struct ExecReturnValue {
    pub flags: u32,
    pub data: Vec<u8>,
}

/// The runtime of ink! machine
pub struct Sandbox {
    pub input: Option<Vec<u8>>,
    pub ret: Option<Vec<u8>>,
    pub instantiates: Vec<instantiate::InstantiateEntry>,
    pub restores: Vec<restore::RestoreEntry>,
    pub rent_allowance: [u8; 32],
    pub terminations: Vec<termination::TerminationEntry>,
    pub transfers: Vec<transfer::TransferEntry>,
    pub schedule: Schedule,
    pub rent_params: RentParams,
    pub gas_meter: GasMeter,
    state: BTreeMap<StorageKey, Vec<u8>>,
    memory: Memory,
    events: Vec<(Vec<[u8; 32]>, Vec<u8>)>,
}

impl Sandbox {
    /// New sandbox
    pub fn new(memory: Memory, state: BTreeMap<StorageKey, Vec<u8>>) -> Sandbox {
        Sandbox {
            input: None,
            ret: None,
            instantiates: vec![],
            restores: vec![],
            rent_allowance: [0; 32],
            terminations: vec![],
            transfers: vec![],
            events: vec![],
            schedule: Default::default(),
            rent_params: Default::default(),
            gas_meter: Default::default(),
            state,
            memory,
        }
    }
}
