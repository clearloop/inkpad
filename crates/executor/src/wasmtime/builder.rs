//! Wasmtime Enviroment
use super::{memory::Memory, util};
use crate::{
    derive::{self, HostFuncType},
    Error,
};
use ceres_std::BTreeMap;
use wasmtime::{Extern, ExternType, ImportType, Store};

/// wasmtime external
pub enum External<T> {
    Memory(Memory),
    Func(HostFuncType<T>),
}

/// wasmtime builder
pub struct Builder<T> {
    pub map: BTreeMap<(Vec<u8>, Vec<u8>), External<T>>,
    pub mem: Option<Memory>,
    pub defined_host_functions: Vec<HostFuncType<T>>,
}

impl<T> Builder<T> {
    /// Get the current store
    pub fn store(&self) -> Option<&Store> {
        if let Some(memory) = &self.mem {
            Some(memory.store())
        } else {
            None
        }
    }

    /// Resolve extern
    pub fn resolve(
        &self,
        store: &Store,
        state: &mut T,
        // Required imports
        required: Vec<ImportType>,
    ) -> Result<Vec<Extern>, Error> {
        let mut imports: Vec<Extern> = vec![];

        let state_ptr = state as *const T as _;
        for ty in required {
            let mut key = (ty.module().as_bytes().to_owned(), vec![]);
            if let Some(name) = ty.name() {
                key.1 = name.as_bytes().to_owned();
            } else {
                // NOTE: Skip value which has unknown name
                continue;
            }

            let external = self.map.get(&key).ok_or(Error::GetExternalFailed(
                String::from_utf8_lossy(&key.1).to_string(),
            ))?;
            match external {
                External::Func(func) => match ty.ty() {
                    ExternType::Func(sig) => {
                        let fn_ptr = *func as usize;
                        imports.push(Extern::Func(util::wrap_fn::<T>(
                            store, state_ptr, fn_ptr, sig,
                        )));
                    }
                    _ => continue,
                },
                External::Memory(mem) => {
                    imports.push(Extern::Memory(mem.clone().cast()));
                }
            }
        }

        Ok(imports)
    }
}

impl<T> derive::Builder<T> for Builder<T> {
    type Memory = Memory;

    fn new() -> Self {
        Builder {
            map: BTreeMap::new(),
            mem: None,
            defined_host_functions: Vec::new(),
        }
    }

    fn add_host_func<N1, N2>(&mut self, module: N1, field: N2, f: HostFuncType<T>)
    where
        N1: Into<Vec<u8>>,
        N2: Into<Vec<u8>>,
    {
        self.map
            .insert((module.into(), field.into()), External::Func(f));
        self.defined_host_functions.push(f);
    }

    fn add_memory<N1, N2>(&mut self, module: N1, field: N2, mem: Memory)
    where
        N1: Into<Vec<u8>>,
        N2: Into<Vec<u8>>,
    {
        self.mem = Some(mem.clone());
        self.map
            .insert((module.into(), field.into()), External::Memory(mem));
    }
}
