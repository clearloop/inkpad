//! WASM executor wrapper
use crate::{
    derive::SealCall, memory::scan_imports, result::ExecResult, Builder, Error, Instance, Memory,
    Result, Value,
};
use ceres_std::Vec;
use parity_wasm::elements::Module;

/// Ceres WASM executor
pub struct Executor<T> {
    instance: Instance<T>,
}

impl<T> Executor<T> {
    /// New executor
    pub fn new(b: &[u8], sandbox: &mut T, ri: Vec<SealCall<T>>) -> Result<Self> {
        let mut el = Module::from_bytes(b).map_err(|_| Error::ParseWasmModuleFailed)?;
        if el.has_names_section() {
            el = match el.parse_names() {
                Ok(m) => m,
                Err((_, m)) => m,
            }
        }

        // construct interfaces
        let mut builder = Builder::new().add_host_parcels(ri);
        let limit = scan_imports(&el).map_err(|_| Error::CalcuateMemoryLimitFailed)?;

        // construct memory
        let memory = Memory::new(limit.0, limit.1)?;
        builder.add_memory("env", "memory", memory.clone());

        // new executor
        Ok(Self {
            instance: Instance::new(
                &el.to_bytes().map_err(|_| Error::ParseWasmModuleFailed)?,
                &builder,
                sandbox,
            )?,
        })
    }

    // invoke method
    pub fn invoke(&mut self, method: &str, data: &[Value], sandbox: &mut T) -> Result<ExecResult> {
        ExecResult::from_res(self.instance.invoke(method, data, sandbox))
    }
}
