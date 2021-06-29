//! Contract executor
use ceres_executor::{derive::SealCall, Builder, Error, ExecResult, Instance, Result};
use ceres_sandbox::Sandbox;
use ceres_std::Vec;
use ceres_support::traits::Executor;
use parity_wasm::elements::Module;

/// Contract executor
#[derive(Default)]
pub struct InkExecutor {
    pub instance: Option<Instance<Sandbox>>,
}

impl Executor<Sandbox, SealCall<Sandbox>, ExecResult, Error> for InkExecutor {
    fn build(&mut self, b: &[u8], sandbox: &mut Sandbox, ri: Vec<SealCall<Sandbox>>) -> Result<()> {
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
            &el.to_bytes().map_err(|_| Error::ParseWasmModuleFailed)?,
            &builder,
            sandbox,
        )?;

        self.instance = Some(instance);
        Ok(())
    }

    fn invoke(&mut self, method: &str, sandbox: &mut Sandbox) -> Result<(Vec<u8>, ExecResult)> {
        if let Some(instance) = self.instance.as_mut() {
            // sandbox.input = Some(data);

            // check return value
            let data = ExecResult::from_res(instance.invoke(method, &[], sandbox))?;
            sandbox.flush_bucket()?;

            // // set return data
            // if let Some(ret) = sandbox.ret.take() {
            //     Ok((ret, data))
            // } else {
            //     Ok((vec![], data))
            // }
            Ok((vec![], data))
        } else {
            Err(Error::ExecutorNotInited)
        }
    }
}
