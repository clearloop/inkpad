//! Ceres Runtimep
use crate::{storage::MemoryStorage, util, Error, Metadata, Result, Storage};
use ceres_executor::{Builder, Instance, Memory};
use ceres_sandbox::{Sandbox, Transaction};
use ceres_std::{Rc, String, ToString, Vec};
use core::cell::RefCell;
use parity_wasm::elements::Module;

/// Ceres Runtime
pub struct Runtime {
    pub sandbox: Rc<RefCell<Sandbox>>,
    instance: Instance<Sandbox>,
    pub metadata: Metadata,
    storage: Box<dyn Storage>,
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

    /// Create runtime from contract
    pub fn from_contract_and_storage(
        contract: &[u8],
        storage: impl Storage + 'static,
    ) -> Result<Runtime> {
        let meta = serde_json::from_str::<Metadata>(&String::from_utf8_lossy(contract))
            .map_err(|_| Error::DecodeContractFailed)?;

        Ok(Self::new(
            &hex::decode(&meta.source.wasm.as_bytes()[2..])
                .map_err(|_| Error::DecodeContractFailed)?,
            meta,
            storage,
        )?)
    }

    /// Create runtime from contract
    pub fn from_metadata_and_storage(
        meta: Metadata,
        storage: impl Storage + 'static,
    ) -> Result<Runtime> {
        Ok(Self::new(
            &hex::decode(&meta.source.wasm.as_bytes()[2..])
                .map_err(|_| Error::DecodeContractFailed)?,
            meta,
            storage,
        )?)
    }

    /// New runtime
    pub fn new(b: &[u8], metadata: Metadata, storage: impl Storage + 'static) -> Result<Runtime> {
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

        // Get storage
        let state = if let Some(state) = storage.get(util::parse_code_hash(&metadata.source.hash)?)
        {
            state.clone()
        } else {
            storage.new_state()
        };

        // Create Sandbox and Builder
        let sandbox = Rc::new(RefCell::new(Sandbox::new(mem, state)));

        // Construct interfaces
        cfg_if::cfg_if! {
            if #[cfg(not(feature = "std"))] {
                let mut builder = Builder::new().add_host_parcels(ceres_seal::pallet_contracts(
                    ceres_seal::NoRuntimeInterfaces,
                ));
            } else {
                let mut builder = Builder::new().add_host_parcels(ceres_seal::pallet_contracts(
                    ceres_ri::Instance
                ));
            }
        }

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
            storage: Box::new(storage),
        })
    }

    /// Deploy contract
    pub fn deploy(&mut self, method: &str, args: &[&str], tx: Option<Transaction>) -> Result<()> {
        if let Some(tx) = tx {
            self.sandbox.borrow_mut().tx = tx;
        }

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
    pub fn call(
        &mut self,
        method: &str,
        args: &[&str],
        tx: Option<Transaction>,
    ) -> Result<Vec<u8>> {
        if let Some(tx) = tx {
            self.sandbox.borrow_mut().tx = tx;
        }

        let messages = self.metadata.messages();
        let (selector, tys) = messages.get(method).ok_or(Error::GetMethodFailed {
            name: method.to_string(),
        })?;

        let mut bm = self.sandbox.borrow_mut();
        bm.input = Some(util::parse_args(selector, args, tys.to_vec())?);

        let res = self.instance.invoke("call", &[], &mut bm);
        if let Some(ret) = bm.ret.take() {
            return Ok(ret);
        } else {
            res.map_err(|e| Error::CallContractFailed {
                error: format!("{:?}", e),
            })?;
        }

        Ok(vec![])
    }

    /// Flush storage
    pub fn flush(&mut self) -> Result<()> {
        self.storage.set(
            util::parse_code_hash(&self.metadata.source.hash)?,
            self.sandbox.borrow().state.clone(),
        )?;

        Ok(())
    }
}
