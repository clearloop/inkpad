//! Ceres Runtime
use crate::{method::InkMethod, util, Error, InkExecutor, Metadata, Result};
use ceres_executor::Memory;
use ceres_sandbox::{RuntimeInterfaces, Sandbox, Transaction};
use ceres_std::{Rc, String, ToString, Vec};
use ceres_support::{
    traits::{Executor, Storage},
    types::MemoryStorage,
};
use core::cell::RefCell;
use parity_wasm::elements::Module;

/// Ceres Runtime
pub struct Runtime {
    pub sandbox: Sandbox,
    pub executor: Rc<RefCell<InkExecutor>>,
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

        // get code hash
        let code_hash = util::parse_code_hash(&metadata.source.hash)?;

        // Set memory
        let limit = util::scan_imports(&el).map_err(|_| Error::CalcuateMemoryLimitFailed)?;
        let mem = Memory::new(limit.0, limit.1).map_err(|_| Error::AllocMemoryFailed)?;

        // Construct seal calls
        let seal_calls = ceres_seal::pallet_contracts(ri);

        // Create Sandbox and Builder
        let mut sandbox = Sandbox::new(
            code_hash,
            mem,
            cache.clone(),
            state.clone(),
            seal_calls.clone(),
            Rc::new(RefCell::new(InkExecutor::default())),
        );

        // Store contract
        let contract = &el
            .to_bytes()
            .map_err(|error| Error::SerializeFailed { error })?;
        cache
            .borrow_mut()
            .set(code_hash, contract.to_vec())
            .ok_or(Error::CouldNotSetStorage)?;

        // Construct executor
        let executor = Rc::new(RefCell::new(InkExecutor::default()));
        executor
            .borrow_mut()
            .build(&contract, &mut sandbox, seal_calls)
            .map_err(|error| Error::InitModuleFailed { error })?;

        Ok(Runtime {
            sandbox,
            executor,
            metadata,
            cache,
            state,
        })
    }

    /// Deploy contract
    pub fn deploy(
        &mut self,
        method: &str,
        args: Vec<Vec<u8>>,
        tx: Option<Transaction>,
    ) -> Result<Option<Vec<u8>>> {
        self.invoke(InkMethod::Deploy, method, args, tx)
    }

    /// Call contract
    pub fn call(
        &mut self,
        method: &str,
        args: Vec<Vec<u8>>,
        tx: Option<Transaction>,
    ) -> Result<Option<Vec<u8>>> {
        self.invoke(InkMethod::Call, method, args, tx)
    }

    // Invoke (ink) method
    pub fn invoke(
        &mut self,
        method: InkMethod,
        inner_method: &str,
        args: Vec<Vec<u8>>,
        tx: Option<Transaction>,
    ) -> Result<Option<Vec<u8>>> {
        if let Some(tx) = tx {
            self.sandbox.tx = tx;
        }

        // set input
        self.sandbox.input = Some(method.parse(&self.metadata, inner_method, args)?);

        self.executor
            .borrow_mut()
            .invoke(&method.to_string(), &mut self.sandbox)
            .map_err(|error| Error::CallContractFailed { error })?;

        Ok(self.sandbox.ret.clone())
    }
}
