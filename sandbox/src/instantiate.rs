//! Instantiate Entry
use crate::{ExecReturnValue, GasMeter, Sandbox};
use ceres_executor::Result;
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
        endowment: u64,
        gas_meter: &mut GasMeter,
        data: Vec<u8>,
        salt: &[u8],
    ) -> Result<([u8; 32], ExecReturnValue, u32)> {
        self.ext.instantiates.push(InstantiateEntry {
            code_hash: code_hash.clone(),
            endowment,
            data: data.to_vec(),
            gas_left: gas_meter.gas_left,
            salt: salt.to_vec(),
        });
        Ok((
            code_hash,
            ExecReturnValue {
                flags: crate::ReturnFlags::empty(),
                data: Vec::new(),
            },
            0,
        ))
    }
}
