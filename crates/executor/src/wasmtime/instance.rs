//! Wasmtime Instance
use super::{builder::Builder, util};
use crate::{
    derive::{self, Value},
    Error,
};
use wasmtime::{Extern, Global, Instance as InstanceRef, Module, Val};

fn extern_global(extern_: &Extern) -> Option<&Global> {
    match extern_ {
        Extern::Global(glob) => Some(glob),
        _ => None,
    }
}

/// wasmtime instance
pub struct Instance<T> {
    instance: InstanceRef,
    _marker: std::marker::PhantomData<T>,
}

impl<T> derive::Instance<T> for Instance<T> {
    type Builder = Builder<T>;

    fn new(code: &[u8], env_def_builder: &Builder<T>, state: &mut T) -> Result<Instance<T>, Error> {
        let dummy_store = util::store_with_dwarf()?;
        let store = if let Some(store) = env_def_builder.store() {
            store
        } else {
            &dummy_store
        };
        let module =
            Module::from_binary(store.engine(), code).map_err(|_| Error::InitModuleFailed)?;
        let imports =
            env_def_builder.resolve(store, state, module.imports().collect::<Vec<_>>())?;
        let instance =
            InstanceRef::new(store, &module, &imports).map_err(|_| Error::InitModuleFailed)?;

        Ok(Instance {
            instance,
            _marker: std::marker::PhantomData::<T>,
        })
    }

    fn invoke(
        &mut self,
        name: &str,
        args: &[Value],
        // The ptr of this state(Runtime in pallet contract) has been packed
        // into closures while generating the WASM module.
        _state: &mut T,
    ) -> Result<Value, Error> {
        let args = args.iter().cloned().map(util::to_val).collect::<Vec<_>>();
        let func = self
            .instance
            .get_func(name)
            .ok_or(Error::GetFunctionNameFailed)?;
        match func.call(&args) {
            Ok(result) => {
                if result.len() != 1 {
                    return Err(Error::UnExpectedReturnValue);
                } else {
                    Ok(util::from_val(result[0].clone()))
                }
            }
            Err(e) => Err(if let Ok(trap) = e.downcast::<::wasmtime::Trap>() {
                Error::from(trap)
            } else {
                Error::UnkownError
            }),
        }
    }

    fn get_global_val(&self, name: &str) -> Option<Value> {
        let global = match self.instance.get_export(name) {
            Some(global) => global,
            None => return None,
        };

        let global = extern_global(&global)
            .ok_or_else(|| format!("`{}` is not a global", name))
            .ok()?;

        match global.get() {
            Val::I32(val) => Some(Value::I32(val)),
            Val::I64(val) => Some(Value::I64(val)),
            Val::F32(val) => Some(Value::F32(val)),
            Val::F64(val) => Some(Value::F64(val)),
            _ => None,
        }
    }
}
