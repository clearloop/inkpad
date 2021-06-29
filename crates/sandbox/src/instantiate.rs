//! Instantiate Entry
use crate::{contract::GasMeter, transfer::TransferEntry, Sandbox};
use ceres_executor::{Error, Result, ReturnCode, ReturnData};
use ceres_std::Vec;

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

        // Get contract from code_hash
        //
        // entrypoint
        let contract = &mut self
            .cache
            .borrow()
            .get(code_hash)
            .ok_or(Error::ExecuteFailed(ReturnCode::CodeNotFound))?;

        // Call deploy by provided `data`
        let executor = self.executor.clone();
        let mut executor_mut = executor.borrow_mut();
        executor_mut.build(&contract, self, self.ri.clone())?;

        self.input = Some(data);
        let ret = executor_mut.invoke("deploy", self)?;

        // return data
        Ok((code_hash, ret.1.data))
    }

    /// Call other contract
    pub fn call(&mut self, code_hash: [u8; 32], value: u64, data: Vec<u8>) -> Result<ReturnData> {
        self.ext.transfers.push(TransferEntry {
            to: code_hash,
            value,
            data: data.clone(),
        });

        // Get contract from code_hash
        //
        // entrypoint
        let contract = &mut self
            .cache
            .borrow()
            .get(code_hash)
            .ok_or(Error::ExecuteFailed(ReturnCode::CodeNotFound))?;

        // Call deploy by provided `data`
        let executor = self.executor.clone();
        let mut executor_mut = executor.borrow_mut();
        executor_mut.build(&contract, self, self.ri.clone())?;

        self.input = Some(data);
        let ret = executor_mut.invoke("call", self)?;

        // return data
        Ok(ret.1.data)
    }
}
