//! WASMI env builder
use super::{
    func::{DefinedHostFunctions, HostFuncIndex},
    memory::Memory,
};
use crate::derive;
use ::wasmi::{
    Error, FuncInstance, FuncRef, GlobalDescriptor, GlobalRef, ImportResolver, MemoryDescriptor,
    MemoryRef, Signature, TableDescriptor, TableRef,
};
use inkpad_std::{format, BTreeMap, String, ToOwned, Vec};

enum ExternVal {
    HostFunc(HostFuncIndex),
    Memory(super::memory::Memory),
}

/// WASMI env builder
pub struct Builder<T> {
    map: BTreeMap<(Vec<u8>, Vec<u8>), ExternVal>,
    /// Defined hos functions
    pub defined_host_functions: DefinedHostFunctions<T>,
}

impl<T> derive::Builder<T> for Builder<T> {
    type Memory = Memory;

    fn new() -> Self {
        Builder {
            map: BTreeMap::new(),
            defined_host_functions: DefinedHostFunctions::new(),
        }
    }

    fn add_host_func<M, F>(&mut self, module: M, field: F, f: derive::HostFuncType<T>)
    where
        F: Into<Vec<u8>>,
        M: Into<Vec<u8>>,
    {
        let idx = self.defined_host_functions.define(f);
        self.map
            .insert((module.into(), field.into()), ExternVal::HostFunc(idx));
    }

    fn add_memory<M, F>(&mut self, module: M, field: F, mem: Memory)
    where
        M: Into<Vec<u8>>,
        F: Into<Vec<u8>>,
    {
        self.map
            .insert((module.into(), field.into()), ExternVal::Memory(mem));
    }
}

impl<T> ImportResolver for Builder<T> {
    fn resolve_func(
        &self,
        module_name: &str,
        field_name: &str,
        signature: &Signature,
    ) -> Result<FuncRef, Error> {
        let key = (
            module_name.as_bytes().to_owned(),
            field_name.as_bytes().to_owned(),
        );
        let externval = self.map.get(&key).ok_or_else(|| {
            Error::Instantiation(format!("Export {}:{} not found", module_name, field_name))
        })?;
        let host_func_idx = match *externval {
            ExternVal::HostFunc(ref idx) => idx,
            _ => {
                return Err(Error::Instantiation(format!(
                    "Export {}:{} is not a host func",
                    module_name, field_name
                )))
            }
        };
        Ok(FuncInstance::alloc_host(signature.clone(), host_func_idx.0))
    }

    fn resolve_global(
        &self,
        _module_name: &str,
        _field_name: &str,
        _global_type: &GlobalDescriptor,
    ) -> Result<GlobalRef, Error> {
        Err(Error::Instantiation(String::from(
            "Importing globals is not supported yet",
        )))
    }

    fn resolve_memory(
        &self,
        module_name: &str,
        field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        let key = (
            module_name.as_bytes().to_owned(),
            field_name.as_bytes().to_owned(),
        );
        let externval = self.map.get(&key).ok_or_else(|| {
            Error::Instantiation(format!("Export {}:{} not found", module_name, field_name))
        })?;

        let memory = match *externval {
            ExternVal::Memory(ref m) => m,
            _ => {
                return Err(Error::Instantiation(format!(
                    "Export {}:{} is not a memory",
                    module_name, field_name
                )))
            }
        };
        Ok(memory.0.clone())
    }

    fn resolve_table(
        &self,
        _module_name: &str,
        _field_name: &str,
        _table_type: &TableDescriptor,
    ) -> Result<TableRef, Error> {
        Err(Error::Instantiation(String::from(
            "Importing tables is not supported yet",
        )))
    }
}
