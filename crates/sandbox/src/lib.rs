#![cfg_attr(not(feature = "std"), no_std)]
use self::ext::Ext;
use ceres_executor::{derive::SealCall, Error, Memory, Result};
use ceres_std::{vec, Rc, Vec};
use ceres_support::{
    convert,
    traits::{self, Frame},
    types::Metadata,
};
use core::cell::RefCell;
use parity_scale_codec::Encode;
use parity_wasm::elements::Module;

/// Custom storage key
pub type StorageKey = [u8; 32];

mod chain;
mod contract;
mod crypto;
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
    pub cache: Rc<RefCell<dyn Frame<Memory>>>,
    pub events: Vec<(Vec<[u8; 32]>, Vec<u8>)>,
    pub ri: Vec<SealCall<Self>>,
}

impl Sandbox {
    /// New sandbox
    pub fn new(
        cache: Rc<RefCell<impl Frame<Memory> + 'static>>,
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

    /// Preare a new frame
    pub fn prepare(&mut self, code_hash: [u8; 32]) -> Result<()> {
        let mut cache_mut = self.cache.borrow_mut();
        if cache_mut.switch(code_hash).is_none() {
            let contract = cache_mut.get(&code_hash).ok_or(Error::CodeNotFound)?;
            let limit = ceres_executor::scan_imports(&Module::from_bytes(&contract)?)?;
            let memory = Memory::new(limit.0, limit.1)?;
            cache_mut.push(code_hash, memory);
        }
        drop(cache_mut);

        Ok(())
    }

    /// Load metadata to cache
    pub fn load_metadata(&mut self, meta: &Metadata) -> Result<[u8; 32]> {
        let code_hash =
            convert::parse_code_hash(&meta.source.hash).ok_or(Error::DecodeContractFailed)?;
        self.cache.borrow_mut().set(
            code_hash.to_vec(),
            Metadata::wasm(&meta.encode()).ok_or(Error::DecodeContractFailed)?,
        );
        Ok(code_hash)
    }
}

impl traits::Ext<Memory, Vec<SealCall<Self>>> for Sandbox {
    fn code(&self, hash: [u8; 32]) -> Option<Vec<u8>> {
        self.cache.borrow().get(&hash).map(|v| v.to_vec())
    }

    fn memory(&self) -> Option<Memory> {
        self.cache.borrow().memory()
    }

    fn seal_call(&self) -> Vec<SealCall<Self>> {
        self.ri.clone()
    }
}
