//! WASM executor wrapper
use crate::{
    derive::SealCall, memory::scan_imports, result::ExecResult, Builder, Error, Instance, Memory,
    Result,
};
use ceres_std::Vec;
use parity_wasm::elements::Module;

/// Ceres WASM executor
pub struct Executor<T> {
    instance: Instance<T>,
}

impl<T> Executor<T> {
    /// Build wasm module
    pub fn build(&mut self, b: &[u8], sandbox: &mut T, ri: Vec<SealCall<T>>) -> Result<()> {
        let mut el = Module::from_bytes(b).map_err(|_| Error::ParseWasmModuleFailed)?;
        if el.has_names_section() {
            el = match el.parse_names() {
                Ok(m) => m,
                Err((_, m)) => m,
            }
        }

        // Construct interfaces
        let mut builder = Builder::new().add_host_parcels(ri);
        let limit = scan_imports(&el).map_err(|_| Error::CalcuateMemoryLimitFailed)?;

        builder.add_memory("env", "memory", Memory::new(limit.0, limit.1)?);

        let instance = Instance::new(
            &el.to_bytes().map_err(|_| Error::ParseWasmModuleFailed)?,
            &builder,
            sandbox,
        )?;

        self.instance = instance;
        Ok(())
    }

    /// Invoke method
    pub fn invoke(&mut self, method: &str, sandbox: &mut T) -> Result<ExecResult> {
        ExecResult::from_res(self.instance.invoke(method, &[], sandbox))
    }
}
