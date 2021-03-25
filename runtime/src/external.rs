use crate::{seal, Sandbox};
use alloc::{prelude::v1::Box, rc::Rc};
use core::cell::RefCell;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, ModuleImportResolver, RuntimeArgs, Signature, TrapKind,
};

/// External functions for executing ink! contract
pub struct HostExternals {
    sandbox: Rc<RefCell<Sandbox>>,
}

impl Externals for HostExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        match index {
            0 => seal::seal_get_storage(self.sandbox.clone(), args),
            1 => seal::seal_set_storage(self.sandbox.clone(), args),
            2 => seal::seal_input(self.sandbox.clone(), args),
            3 => seal::seal_value_transferred(self.sandbox.clone(), args),
            4 => seal::seal_return(self.sandbox.clone(), args),
            _ => panic!("Unimplemented function at {}", index),
        }
        .map_err(|e| wasmi::Trap::from(TrapKind::Host(Box::new(e))))
    }
}

impl ModuleImportResolver for HostExternals {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        match field_name {
            "seal_get_storage" => Ok(FuncInstance::alloc_host(signature.clone(), 0)),
            "seal_set_storage" => Ok(FuncInstance::alloc_host(signature.clone(), 0)),
            "seal_input" => Ok(FuncInstance::alloc_host(signature.clone(), 0)),
            "seal_value_transferred" => Ok(FuncInstance::alloc_host(signature.clone(), 0)),
            "seal_return" => Ok(FuncInstance::alloc_host(signature.clone(), 0)),
            _ => {
                return Err(Error::Instantiation(format!(
                    "Export {} not found",
                    field_name
                )))
            }
        }
    }
}
