//! Ceres Runtime
use crate::{util, Error, Executor, Metadata, Result};
use ceres_executor::Memory;
use ceres_sandbox::{Sandbox, Transaction};
use ceres_seal::RuntimeInterfaces;
use ceres_std::{Rc, String, ToString, Vec};
use ceres_support::{traits::Storage, types::MemoryStorage};
use core::cell::RefCell;
use parity_wasm::elements::Module;

/// Ceres Runtime
pub struct Runtime {
    pub sandbox: Rc<RefCell<Sandbox>>,
    pub executor: Rc<RefCell<Executor>>,
    pub metadata: Metadata,
    cache: Rc<RefCell<dyn Storage>>,
    state: Rc<RefCell<dyn Storage>>,
}

impl Runtime {
    /// Create runtime from contract
    pub fn from_contract(contract: &[u8], ri: Option<impl RuntimeInterfaces>) -> Result<Runtime> {
        let meta = serde_json::from_str::<Metadata>(&String::from_utf8_lossy(contract))
            .map_err(|_| Error::DecodeContractFailed)?;

        Self::new(
            &hex::decode(&meta.source.wasm.as_bytes()[2..])
                .map_err(|_| Error::DecodeContractFailed)?,
            meta,
            Rc::new(RefCell::new(MemoryStorage::default())),
            Rc::new(RefCell::new(MemoryStorage::default())),
            ri,
        )
    }

    /// Create runtime from contract
    pub fn from_contract_and_storage(
        contract: &[u8],
        cache: Rc<RefCell<impl Storage + 'static>>,
        state: Rc<RefCell<impl Storage + 'static>>,
        ri: Option<impl RuntimeInterfaces>,
    ) -> Result<Runtime> {
        let meta = serde_json::from_str::<Metadata>(&String::from_utf8_lossy(contract))
            .map_err(|_| Error::DecodeContractFailed)?;

        Self::new(
            &hex::decode(&meta.source.wasm.as_bytes()[2..])
                .map_err(|_| Error::DecodeContractFailed)?,
            meta,
            cache,
            state,
            ri,
        )
    }

    /// Create runtime from metadata and storage
    pub fn from_metadata_and_storage(
        meta: Metadata,
        cache: Rc<RefCell<impl Storage + 'static>>,
        state: Rc<RefCell<impl Storage + 'static>>,
        ri: Option<impl RuntimeInterfaces>,
    ) -> Result<Runtime> {
        Self::new(
            &hex::decode(&meta.source.wasm.as_bytes()[2..])
                .map_err(|_| Error::DecodeContractFailed)?,
            meta,
            cache,
            state,
            ri,
        )
    }

    /// New runtime
    pub fn new(
        b: &[u8],
        metadata: Metadata,
        cache: Rc<RefCell<impl Storage + 'static>>,
        state: Rc<RefCell<impl Storage + 'static>>,
        ri: Option<impl RuntimeInterfaces>,
    ) -> Result<Runtime> {
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

        // Construct seal calls
        let seal_calls = ceres_seal::pallet_contracts(ri);

        // Create Sandbox and Builder
        let sandbox = Rc::new(RefCell::new(Sandbox::new(
            mem,
            cache.clone(),
            state.clone(),
            seal_calls.clone(),
        )));

        // Construct executor
        let executor = Rc::new(RefCell::new(Executor::default()));

        executor.borrow_mut().build(
            &el.to_bytes()
                .map_err(|error| Error::SerializeFailed { error })?,
            &mut sandbox.borrow_mut(),
            seal_calls,
        )?;

        Ok(Runtime {
            sandbox,
            executor,
            metadata,
            cache,
            state,
        })
    }

    /// Deploy contract
    pub fn deploy(&self, method: &str, args: Vec<Vec<u8>>, tx: Option<Transaction>) -> Result<()> {
        if let Some(tx) = tx {
            self.sandbox.borrow_mut().tx = tx;
        }

        let constructors = self.metadata.constructors();
        let (selector, tys) = constructors.get(method).ok_or(Error::GetMethodFailed {
            name: method.to_string(),
        })?;

        self.executor.borrow_mut().invoke(
            "deploy",
            util::parse_args(selector, args, tys.iter().map(|ty| ty.1).collect())?,
            &mut self.sandbox.borrow_mut(),
        )?;

        Ok(())
    }

    /// Call contract
    pub fn call(
        &self,
        method: &str,
        args: Vec<Vec<u8>>,
        tx: Option<Transaction>,
    ) -> Result<Vec<u8>> {
        if let Some(tx) = tx {
            self.sandbox.borrow_mut().tx = tx;
        }

        let messages = self.metadata.messages();
        let (selector, tys) = messages.get(method).ok_or(Error::GetMethodFailed {
            name: method.to_string(),
        })?;

        Ok(self.executor.borrow_mut().invoke(
            "call",
            util::parse_args(selector, args, tys.iter().map(|ty| ty.1).collect())?,
            &mut self.sandbox.borrow_mut(),
        )?)
    }
}
