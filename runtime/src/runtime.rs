//! Ceres Runtimep
use crate::{storage::MemoryStorage, util, Error, Metadata, Result, Storage};
use ceres_executor::{Builder, Instance, Memory};
use ceres_sandbox::Sandbox;
use ceres_std::{Rc, String, ToString, Vec};
use core::cell::RefCell;
use parity_wasm::elements::Module;

/// Ceres Runtime
pub struct Runtime {
    sandbox: Rc<RefCell<Sandbox>>,
    instance: Instance<Sandbox>,
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
            MemoryStorage::new(),
        )?)
    }

    /// New runtime
    pub fn new(b: &[u8], metadata: Metadata, storage: impl Storage) -> Result<Runtime> {
        let mut el = Module::from_bytes(b).map_err(|_| Error::ParseWasmModuleFailed)?;
        if el.has_names_section() {
            el = match el.parse_names() {
                Ok(m) => m,
                Err((_, m)) => m,
            }
        }

        // Set memory
        let limit = util::scan_imports(&el).map_err(|_| Error::CalcuateMemoryLimitFailed)?;
        let mem = Memory::new(limit.0, limit.1).map_err(|_| Error::AllocMemoryFailed)?;

        // get storage
        let state = if let Some(state) = storage.get(util::parse_code_hash(&metadata.source.hash)?)
        {
            state.clone()
        } else {
            storage.new_state()
        };

        // Create Sandbox and Builder
        let sandbox = Rc::new(RefCell::new(Sandbox::new(mem, state)));
        let mut builder = Builder::new().add_host_parcels(ceres_seal::pallet_contracts(
            ceres_seal::NoRuntimeInterfaces,
        ));

        // **Note**
        //
        // The memory is `cloned()`, trying using one memory.
        builder.add_memory("env", "memory", sandbox.borrow().mem());

        // Create instance
        let instance = Instance::new(
            &el.to_bytes().map_err(|_| Error::InitModuleFailed)?,
            &builder,
            &mut sandbox.borrow_mut(),
        )
        .map_err(|_| Error::InitModuleFailed)?;

        Ok(Runtime {
            instance,
            metadata,
            sandbox,
        })
    }

    /// Deploy contract
    pub fn deploy(&mut self, method: &str, args: &[&str]) -> Result<()> {
        let constructors = self.metadata.constructors();
        let (selector, tys) = constructors.get(method).ok_or(Error::GetMethodFailed {
            name: method.to_string(),
        })?;

        let mut bm = self.sandbox.borrow_mut();
        bm.input = Some(util::parse_args(selector, args, tys.to_vec())?);
        self.instance
            .invoke("deploy", &[], &mut bm)
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
        let res = self
            .instance
            .invoke("call", &[], &mut self.sandbox.borrow_mut());
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
