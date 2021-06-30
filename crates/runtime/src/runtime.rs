//! Ceres Runtime
use crate::{method::InkMethod, util, Error, Metadata, Result};
use ceres_executor::{derive::SealCall, Executor, Memory};
use ceres_sandbox::{RuntimeInterfaces, Sandbox, Transaction};
use ceres_std::{Box, Rc, String, ToString, Vec};
use ceres_support::{traits, types};
use core::cell::RefCell;
use parity_wasm::elements::Module;

/// Runtime cache
pub type Cache = traits::Cache<Memory>;

/// Ceres Runtime
pub struct Runtime {
    pub sandbox: Sandbox,
    pub metadata: Metadata,
    cache: Rc<RefCell<Cache>>,
    executor: Executor<Sandbox>,
    ri: Vec<SealCall<Sandbox>>,
}

impl Runtime {
    /// Create runtime from contract
    pub fn contract(contract: &[u8], ri: Option<impl RuntimeInterfaces>) -> Result<Runtime> {
        let meta = serde_json::from_str::<Metadata>(&String::from_utf8_lossy(contract))
            .map_err(|_| Error::DecodeContractFailed)?;

        Self::new(
            &hex::decode(&meta.source.wasm.as_bytes()[2..])
                .map_err(|_| Error::DecodeContractFailed)?,
            meta,
            MemoryStorage::default(),
            MemoryStorage::default(),
            ri,
        )
    }

    /// Create runtime from contract
    pub fn from_contract(
        contract: &[u8],
        cache: impl Cache + 'static,
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
    pub fn from_metadata(
        meta: Metadata,
        cache: impl Cache + 'static,
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
        cache: impl Storage + 'static,
        state: impl Storage + 'static,
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

        // generate seal calls
        let seal_calls = ceres_seal::pallet_contracts(ri);

        // Create Sandbox and Builder
        let mut sandbox = Sandbox::new(code_hash, cache, seal_calls.clone());

        // Store contract
        let contract = &el
            .to_bytes()
            .map_err(|error| Error::SerializeFailed { error })?;
        cache
            .set(code_hash, contract.to_vec())
            .ok_or(Error::CouldNotSetStorage)?;

        // Init executor
        let executor = Executor::new(contract, &mut sandbox, seal_calls)
            .map_err(|_| Error::InitExecutorFailed)?;

        Ok(Runtime {
            sandbox,
            metadata,
            cache: Box::new(cache),
            state: Box::new(MemoryStorage::default()),
            executor,
            ri: ceres_seal::pallet_contracts(ri),
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
            .invoke(&method.to_string(), &[], &mut self.sandbox)
            .map_err(|error| Error::CallContractFailed { error })?;

        Ok(self.sandbox.ret.clone())
    }
}
