//! Contract executor
use crate::result::{Error, Result};
use ceres_executor::{derive::SealCall, Builder, Instance};
use ceres_sandbox::Sandbox;
use parity_wasm::elements::Module;

/// Contract executor
#[derive(Default)]
pub struct Executor {
    pub instance: Option<Instance<Sandbox>>,
}

impl Executor {
    /// build instance
    pub fn build(
        &mut self,
        b: &[u8],
        sandbox: &mut Sandbox,
        ri: Vec<SealCall<Sandbox>>,
    ) -> Result<()> {
        let mut el = Module::from_bytes(b).map_err(|_| Error::ParseWasmModuleFailed)?;
        if el.has_names_section() {
            el = match el.parse_names() {
                Ok(m) => m,
                Err((_, m)) => m,
            }
        }

        // Construct interfaces
        let mut builder = Builder::new().add_host_parcels(ri);
        builder.add_memory("env", "memory", sandbox.mem());

        let instance = Instance::new(
            &el.to_bytes()
                .map_err(|error| Error::SerializeFailed { error })?,
            &builder,
            sandbox,
        )
        .map_err(|error| Error::InitModuleFailed { error })?;

        self.instance = Some(instance);
        Ok(())
    }

    /// Call a method
    pub fn invoke(
        &mut self,
        method: &str,
        data: Vec<u8>,
        sandbox: &mut Sandbox,
    ) -> Result<Vec<u8>> {
        if let Some(instance) = self.instance.as_mut() {
            sandbox.input = Some(data);
            let res = instance.invoke(method, &[], sandbox);
            if let Some(ret) = sandbox.ret.take() {
                return Ok(ret);
            } else {
                res.map_err(|error| Error::CallContractFailed { error })?;
            }

            Ok(vec![])
        } else {
            Err(Error::ExecutorNotInited)
        }
    }
}
