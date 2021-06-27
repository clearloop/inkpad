//! Instantiate Entry
use crate::{contract::GasMeter, Sandbox};
use ceres_executor::{Error, Result, ReturnCode, ReturnData};
use ceres_std::Vec;

/// Instantiate Entry
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
    ) -> Result<([u8; 32], ReturnData, u32)> {
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
        let ret = executor_mut.invoke("deploy", data, self)?;

        // return data
        Ok((code_hash, ret.1.data, ret.1.value.as_u32()))
    }
}
