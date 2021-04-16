//! WASMi instance
use super::{builder::Builder, external::External, func::DefinedHostFunctions};
use crate::{
    derive::{self, ReturnValue, Value},
    Error, Result,
};
use ::wasmi::{Module, ModuleInstance, ModuleRef};
use ceres_std::Vec;

/// WASMi instance
pub struct Instance<T> {
    instance: ModuleRef,
    defined_host_functions: DefinedHostFunctions<T>,
}

impl<T> derive::Instance<T> for Instance<T> {
    type Builder = Builder<T>;

    fn new(code: &[u8], builder: &Self::Builder, state: &mut T) -> Result<Self> {
        let module = Module::from_buffer(code)
            .map_err(|_| Error::InitModuleFailed)?
            .try_parse_names();
        let not_started_instance =
            ModuleInstance::new(&module, builder).map_err(|_| Error::InitModuleFailed)?;

        let defined_host_functions = builder.defined_host_functions.clone();
        let instance = {
            let mut externals = External {
                state,
                defined_host_functions: &defined_host_functions,
            };
            let instance = not_started_instance
                .run_start(&mut externals)
                .map_err(|_| Error::ExecuteFailed)?;
            instance
        };

        Ok(Instance {
            instance,
            defined_host_functions,
        })
    }

    fn invoke(&mut self, name: &str, args: &[Value], state: &mut T) -> Result<ReturnValue> {
        let args = args.iter().cloned().map(|v| v.into()).collect::<Vec<_>>();
        let mut externals = External {
            state,
            defined_host_functions: &self.defined_host_functions,
        };
        let result = self.instance.invoke_export(&name, &args, &mut externals);

        match result {
            Ok(None) => Ok(ReturnValue::Unit),
            Ok(Some(v)) => Ok(v.into()),
            Err(e) => Err(match e {
                ::wasmi::Error::Trap(t) => Error::Trap(t.into()),
                _ => Error::ExecuteFailed,
            }),
        }
    }

    fn get_global_val(&self, name: &str) -> Option<Value> {
        Some(
            self.instance
                .export_by_name(name)?
                .as_global()?
                .get()
                .into(),
        )
    }
}
