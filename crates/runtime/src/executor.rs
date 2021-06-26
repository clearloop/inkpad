//! Contract executor
use ceres_executor::{
    derive::SealCall, Builder, Error, Instance, Result, ReturnData, ReturnValue, Value,
};
use ceres_sandbox::Sandbox;
use ceres_support::traits::Executor;
use parity_wasm::elements::Module;

/// Contract executor
#[derive(Default)]
pub struct InkExecutor {
    pub instance: Option<Instance<Sandbox>>,
}

impl Executor<Sandbox, SealCall<Sandbox>, ReturnData, Error> for InkExecutor {
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

    fn invoke(
        &mut self,
        method: &str,
        data: Vec<u8>,
        sandbox: &mut Sandbox,
    ) -> Result<(Vec<u8>, ReturnData)> {
        if let Some(instance) = self.instance.as_mut() {
            sandbox.input = Some(data);

            // check return value
            let data = match instance.invoke(method, &[], sandbox) {
                Ok(res) => match res {
                    ReturnValue::Unit | ReturnValue::Value(Value::I32(0)) => {
                        Ok(ReturnData::default())
                    }
                    ReturnValue::Value(Value::I32(n)) => Err(Error::ExecuteFailed(n.into())),
                    ReturnValue::Value(_) => Err(Error::UnExpectedReturnValue),
                }?,
                Err(Error::Return(ret)) => ret,
                Err(e) => return Err(e),
            };

            // set return data
            if let Some(ret) = sandbox.ret.take() {
                Ok((ret, data))
            } else {
                Ok((vec![], data))
            }
        } else {
            Err(Error::ExecutorNotInited)
        }
    }
}
