//! Ceres Runtime
use crate::{storage::Memory, util, Error, Metadata, Resolver, Result, Sandbox, Storage};
use alloc::rc::Rc;
use core::cell::RefCell;
use parity_wasm::elements::Module as ModuleElement;
use wasmi::{MemoryInstance, Module, ModuleInstance, ModuleRef};

/// Ceres Runtime
pub struct Runtime {
    instance: ModuleRef,
    resolver: Resolver,
    sandbox: Rc<RefCell<Sandbox>>,
    metadata: Metadata,
}

impl Runtime {
    /// Create runtime from contract
    pub fn from_contract(contract: &[u8]) -> Result<Runtime> {
        let meta = serde_json::from_str::<Metadata>(&String::from_utf8_lossy(contract))
            .map_err(|_| Error::DecodeContractFailed)?;

        Ok(Self::new(
            &hex::decode(&meta.source.wasm.as_bytes()[2..])
                .map_err(|_| Error::DecodeContractFailed)?,
            meta,
            Memory::new(),
        )?)
    }

    /// New runtime
    pub fn new(b: &[u8], metadata: Metadata, storage: impl Storage) -> Result<Runtime> {
        let mut el = ModuleElement::from_bytes(b).map_err(|_| Error::ParseWasmModuleFailed)?;
        if el.has_names_section() {
            el = match el.parse_names() {
                Ok(m) => m,
                Err((_, m)) => m,
            }
        }

        // Set memory
        let limit = util::scan_imports(&el).map_err(|_| Error::CalcuateMemoryLimitFailed)?;
        let mem = MemoryInstance::alloc(limit.0, limit.1).map_err(|_| Error::AllocMemoryFailed)?;

        // get storage
        let state = if let Some(state) = storage.get(util::parse_code_hash(&metadata.source.hash)?)
        {
            state.clone()
        } else {
            storage.new_state()
        };

        // Create Sandbox and resolver
        let sandbox = Rc::new(RefCell::new(Sandbox::new(mem, state)));
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
            metadata,
        })
    }

    /// Deploy contract
    pub fn deploy(&mut self, method: &str, args: &[&str]) -> Result<()> {
        let constructors = self.metadata.constructors();
        let (selector, tys) = constructors.get(method).ok_or(Error::GetMethodFailed {
            name: method.to_string(),
        })?;

        self.sandbox.borrow_mut().input = Some(util::parse_args(selector, args, tys.to_vec())?);
        self.instance
            .invoke_export("deploy", &[], &mut self.resolver)
            .map_err(|_| Error::DeployContractFailed)?;

        Ok(())
    }

    /// Call contract
    pub fn call(&mut self, method: &str, args: &[&str]) -> Result<Vec<u8>> {
        let messages = self.metadata.messages();
        let (selector, tys) = messages.get(method).ok_or(Error::GetMethodFailed {
            name: method.to_string(),
        })?;

        self.sandbox.borrow_mut().input = Some(util::parse_args(selector, args, tys.to_vec())?);
        let res = self.instance.invoke_export("call", &[], &mut self.resolver);
        if let Some(ret) = self.sandbox.borrow_mut().ret.take() {
            return Ok(ret);
        } else {
            res.map_err(|e| Error::CallContractFailed {
                error: format!("{:?}", e),
            })?;
        }

        Ok(vec![])
    }
}
