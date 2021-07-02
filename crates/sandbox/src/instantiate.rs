//! Instantiate Entry
use crate::{contract::GasMeter, transfer::TransferEntry, Sandbox};
use ceres_executor::{Error, Executor, Memory, Result, ReturnData};
use ceres_std::Vec;
use ceres_support::{traits::Ext, types::State};
use parity_wasm::elements::Module;

/// Instantiate Entry
#[derive(Default)]
pub struct InstantiateEntry {
    pub code_hash: [u8; 32],
    pub endowment: u64,
    pub data: Vec<u8>,
    pub gas_left: u64,
    pub salt: Vec<u8>,
}

impl Sandbox {
    // Invoke (ink) method
    pub fn invoke(
        &mut self,
        code_hash: [u8; 32],
        method: &str,
        data: Vec<u8>,
    ) -> Result<([u8; 32], ReturnData)> {
        self.input = Some(data);

        // Get memory
        let contract = self.code(code_hash).ok_or(Error::CodeNotFound)?;
        let limit = ceres_executor::scan_imports(&Module::from_bytes(&contract)?)?;
        let memory = Memory::new(limit.0, limit.1)?;

        // Push new state
        self.cache.borrow_mut().push(State::new(code_hash, memory));

        // invoke with provided `data`
        let mut executor = Executor::new(code_hash, self)?;
        let ret = executor.invoke(method, &[], self)?;

        // Pop state
        self.cache.borrow_mut().pop();

        // return vals
        Ok((code_hash, ret.data))
    }

    pub fn instantiate(
        &mut self,
        code_hash: [u8; 32],
        // endowment: u64,
        gas_meter: &mut GasMeter,
        data: Vec<u8>,
        salt: &[u8],
    ) -> Result<([u8; 32], ReturnData)> {
        self.ext.instantiates.push(InstantiateEntry {
            code_hash,
            endowment: 3, // endowment
            data: data.to_vec(),
            gas_left: gas_meter.gas_left,
            salt: salt.to_vec(),
        });

        self.invoke(code_hash, "deploy", data)
    }

    /// Call other contract
    pub fn call(&mut self, code_hash: [u8; 32], value: u64, data: Vec<u8>) -> Result<ReturnData> {
        self.ext.transfers.push(TransferEntry {
            to: code_hash,
            value,
            data: data.clone(),
        });

        self.invoke(code_hash, "deploy", data).map(|v| v.1)
    }
}
