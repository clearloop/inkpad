//! WASMi externals
use super::func::DefinedHostFunctions;
use crate::Error;
use ::wasmi::{Externals, HostError, RuntimeArgs, RuntimeValue, Trap};
use ceres_std::Vec;

impl HostError for Error {}

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

        let res = (self.defined_host_functions.func(index))(self.state, &args)?;
        Ok(res.map(|v| v.into()))
    }
}
