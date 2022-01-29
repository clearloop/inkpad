//! Inkpad Runtime
use crate::{method::InkMethod, Error, Result};
use inkpad_executor::{Executor, Memory};
use inkpad_sandbox::{RuntimeInterfaces, Sandbox, Transaction};
use inkpad_std::{Rc, String, ToString, Vec};
use inkpad_support::{
    convert, traits,
    types::{self, Metadata},
};
use core::cell::RefCell;

/// Inkpad Runtime
pub struct Runtime {
    pub sandbox: Sandbox,
    pub metadata: Metadata,
    pub cache: Rc<RefCell<dyn traits::Frame<Memory>>>,
}

impl Runtime {
    /// Create runtime from contract
    pub fn contract(contract: &[u8], ri: Option<impl RuntimeInterfaces>) -> Result<Runtime> {
        let meta = serde_json::from_str::<Metadata>(&String::from_utf8_lossy(contract))
            .map_err(|_| Error::DecodeContractFailed)?;

        Self::new(meta, types::Cache::default(), ri)
    }

    /// Create runtime from contract
    pub fn from_contract(
        contract: &[u8],
        cache: impl traits::Frame<Memory> + 'static,
        ri: Option<impl RuntimeInterfaces>,
    ) -> Result<Runtime> {
        let meta = serde_json::from_slice::<Metadata>(contract)
            .map_err(|_| Error::DecodeContractFailed)?;

        Self::new(meta, cache, ri)
    }

    /// Create runtime from metadata and storage
    pub fn from_metadata(
        meta: Metadata,
        cache: impl traits::Frame<Memory> + 'static,
        ri: Option<impl RuntimeInterfaces>,
    ) -> Result<Runtime> {
        Self::new(meta, cache, ri)
    }

    /// Load contract to cache
    pub fn load(&mut self, b: &[u8]) -> Result<[u8; 32]> {
        self.load_metadata(
            &serde_json::from_slice::<Metadata>(b).map_err(|_| Error::DecodeContractFailed)?,
        )
    }

    /// Load metadata to cache
    pub fn load_metadata(&mut self, meta: &Metadata) -> Result<[u8; 32]> {
        Ok(self.sandbox.load_metadata(meta)?)
    }

    /// New runtime
    pub fn new(
        metadata: Metadata,
        cache: impl traits::Frame<Memory> + 'static,
        ri: Option<impl RuntimeInterfaces>,
    ) -> Result<Runtime> {
        // generate seal calls
        let seal_calls = inkpad_seal::pallet_contracts(ri);

        // wrap cache
        let cache = Rc::new(RefCell::new(cache));

        // Create Sandbox and Builder
        let mut sandbox = Sandbox::new(cache.clone(), seal_calls.clone());
        let code_hash = sandbox.load_metadata(&metadata)?;
        sandbox.prepare(code_hash)?;

        // Construct runtime
        Ok(Runtime {
            sandbox,
            metadata,
            cache,
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
        // construct transaction
        if let Some(tx) = tx {
            self.sandbox.tx = tx;
        }

        // set input
        self.sandbox.input = Some(method.parse(&self.metadata, inner_method, args)?);

        // execute
        let hash = self
            .cache
            .borrow()
            .active()
            .ok_or(inkpad_executor::Error::CodeNotFound)?;
        Executor::new(
            convert::to_storage_key(&hash[..]).ok_or(inkpad_executor::Error::CodeNotFound)?,
            &mut self.sandbox,
        )?
        .invoke(&method.to_string(), &[], &mut self.sandbox)
        .map_err(|error| Error::CallContractFailed { error })?;

        // flush data
        self.cache
            .borrow_mut()
            .flush()
            .ok_or(Error::FlushDataFailed)?;
        Ok(self.sandbox.ret.take())
    }
}
