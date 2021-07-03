//! WASM executor wrapper
use crate::{
    derive::SealCall, result::ExecResult, Builder, Error, Instance, Memory, Result, Value,
};
use ceres_std::Vec;
use ceres_support::traits::Ext;

/// Ceres WASM executor
pub struct Executor<T> {
    pub memory: Memory,
    instance: Instance<T>,
}

impl<T> Executor<T>
where
    T: Ext<Memory, Vec<SealCall<T>>>,
{
    /// New executor
    pub fn new(code: [u8; 32], sandbox: &mut T) -> Result<Self> {
        // construct builder
        let memory = sandbox.memory().ok_or(Error::MemoryNotFound)?;
        let mut builder = Builder::new().add_host_parcels(sandbox.seal_call());
        builder.add_memory("env", "memory", memory.clone());

        // get wasm code
        let wasm = sandbox.code(code).ok_or(Error::CodeNotFound)?.to_vec();

        // new executor
        Ok(Self {
            memory,
            instance: Instance::new(&wasm, &builder, sandbox)?,
        })
    }

    // invoke method
    pub fn invoke(&mut self, method: &str, data: &[Value], sandbox: &mut T) -> Result<ExecResult> {
        let res = ExecResult::from_res(self.instance.invoke(method, data, sandbox));
        if let Ok(r) = res.clone() {
            log::debug!("{:?}", &r.data);
        }

        res
    }
}
