use crate::{seal, Sandbox};
use ceres_std::{Box, Rc};
use core::cell::RefCell;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, GlobalDescriptor, GlobalRef, ImportResolver,
    MemoryDescriptor, MemoryRef, RuntimeArgs, Signature, TableDescriptor, TableRef, TrapKind,
};

/// External functions for executing ink! contract
pub struct Resolver {
    sandbox: Rc<RefCell<Sandbox>>,
}

impl Resolver {
    /// New externals
    pub fn new(sandbox: Rc<RefCell<Sandbox>>) -> Self {
        Resolver { sandbox }
    }
}

impl Externals for Resolver {
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

impl ImportResolver for Resolver {
    fn resolve_func(
        &self,
        module_name: &str,
        field_name: &str,
        signature: &Signature,
    ) -> Result<FuncRef, Error> {
        match module_name {
            "seal0" => match field_name {
                "seal_get_storage" => Ok(FuncInstance::alloc_host(signature.clone(), 0)),
                "seal_set_storage" => Ok(FuncInstance::alloc_host(signature.clone(), 1)),
                "seal_input" => Ok(FuncInstance::alloc_host(signature.clone(), 2)),
                "seal_value_transferred" => Ok(FuncInstance::alloc_host(signature.clone(), 3)),
                "seal_return" => Ok(FuncInstance::alloc_host(signature.clone(), 4)),
                _ => {
                    return Err(Error::Instantiation(format!(
                        "Export {} not found",
                        field_name
                    )))
                }
            },
            _ => {
                return Err(Error::Instantiation(format!(
                    "Export {} not found",
                    module_name
                )))
            }
        }
    }

    fn resolve_memory(
        &self,
        module_name: &str,
        field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, wasmi::Error> {
        match module_name {
            "env" => match field_name {
                "memory" => Ok(self.sandbox.borrow().mem()),
                _ => Err(wasmi::Error::Instantiation(format!(
                    "Importing memory not found"
                ))),
            },
            _ => Err(wasmi::Error::Instantiation(format!(
                "Importing module not found"
            ))),
        }
    }

    fn resolve_table(
        &self,
        _module_name: &str,
        _field_name: &str,
        _table_type: &TableDescriptor,
    ) -> Result<TableRef, wasmi::Error> {
        Err(wasmi::Error::Instantiation(format!(
            "Importing tables is not supported yet"
        )))
    }

    fn resolve_global(
        &self,
        _module_name: &str,
        _field_name: &str,
        _global_type: &GlobalDescriptor,
    ) -> Result<GlobalRef, wasmi::Error> {
        Err(wasmi::Error::Instantiation(format!(
            "Importing globals is not supported yet"
        )))
    }
}
