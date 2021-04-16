//! WASMi externals
use super::func::DefinedHostFunctions;
use crate::derive::ReturnValue;
use ::wasmi::{Externals, HostError, RuntimeArgs, RuntimeValue, Trap, TrapKind};
use ceres_std::{fmt, Box, Vec};

#[derive(Debug)]
struct DummyHostError;

impl HostError for DummyHostError {}

impl fmt::Display for DummyHostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DummyHostError")
    }
}

/// WASMi externals
pub struct External<'a, T> {
    /// External state
    pub state: &'a mut T,
    /// Defined host functions
    pub defined_host_functions: &'a DefinedHostFunctions<T>,
}

impl<'a, T> Externals for External<'a, T> {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        let args = args
            .as_ref()
            .iter()
            .cloned()
            .map(|v| v.into())
            .collect::<Vec<_>>();

        let result = (self.defined_host_functions.func(index))(self.state, &args);
        match result {
            Ok(value) => Ok(match value {
                ReturnValue::Value(v) => Some(v.into()),
                ReturnValue::Unit => None,
            }),
            // TODO: specified error
            Err(_) => Err(TrapKind::Host(Box::new(DummyHostError)).into()),
        }
    }
}
