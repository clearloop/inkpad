//! Ceres Runtime
use crate::{util, Error, Metadata, Resolver, Result, Sandbox};
use alloc::rc::Rc;
use core::cell::RefCell;
use parity_wasm::elements::Module as ModuleElement;
use wasmi::{MemoryInstance, Module, ModuleInstance, ModuleRef};

/// Ceres Runtime
pub struct Runtime {
    instance: ModuleRef,
    resolver: Resolver,
    sandbox: Rc<RefCell<Sandbox>>,
}

impl Runtime {
    /// Create runtime from contract
    pub fn from_contract(contract: &[u8]) -> Result<Runtime> {
        Ok(Self::new(
            &hex::decode(
                &serde_json::from_str::<Metadata>(&String::from_utf8_lossy(contract))
                    .map_err(|_| Error::DecodeContractFailed)?
                    .source
                    .wasm
                    .as_bytes()[2..],
            )
            .map_err(|_| Error::DecodeContractFailed)?,
        )?)
    }

    /// New runtime
    pub fn new(b: &[u8]) -> Result<Runtime> {
        let mut el = ModuleElement::from_bytes(b).map_err(|_| Error::ParseWasmModuleFailed)?;
        if el.has_names_section() {
            el = el
                .parse_names()
                .map_err(|_| Error::ParseWasmNameSectionFailed)?;
        }

        // Set memory
        let limit = util::scan_imports(&el).map_err(|_| Error::CalcuateMemoryLimitFailed)?;
        let mem = MemoryInstance::alloc(limit.0, limit.1).map_err(|_| Error::AllocMemoryFailed)?;

        // Create Sandbox and resolver
        let sandbox = Rc::new(RefCell::new(Sandbox::new(mem)));
        let resolver = Resolver::new(sandbox.clone());

        // Create instance
        let instance = ModuleInstance::new(
            &Module::from_parity_wasm_module(el).map_err(|_| Error::ParseWasmModuleFailed)?,
            &resolver,
        )
        .map_err(|_| Error::InitModuleFailed)?
        .assert_no_start();

        Ok(Runtime {
            resolver,
            instance,
            sandbox,
        })
    }

    /// Deploy contract
    pub fn deploy(&mut self, input: &str) -> Result<()> {
        self.sandbox.borrow_mut().input = Some(util::step_hex(input)?);
        self.instance
            .invoke_export("deploy", &[], &mut self.resolver)
            .map_err(|_| Error::DeployContractFailed)?;

        Ok(())
    }

    /// Call contract
    pub fn call(&mut self, input: &str) -> Result<()> {
        self.sandbox.borrow_mut().input = Some(util::step_hex(input)?);
        self.instance
            .invoke_export("call", &[], &mut self.resolver)
            .map_err(|_| Error::DeployContractFailed)?;

        Ok(())
    }
}
