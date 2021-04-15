//! Storage implementation
use ceres_runtime::{Error, Metadata, Result, Runtime};
use ceres_std::BTreeMap;
use etc::{Etc, FileSystem, Meta};
use sled::Db;
use std::{fs, path::PathBuf};

/// A ceres storage implementation using sled
pub struct Storage(pub Db);

impl Storage {
    /// New storage
    pub fn new() -> crate::Result<Self> {
        let etc =
            Etc::new(&dirs::home_dir().ok_or(crate::Error::Custom("Could not find home dir"))?)?;

        let storage = Storage(sled::open(etc.open(".ceres/contracts")?.real_path()?)?);
        Ok(storage)
    }

    /// Contract instance
    ///
    /// * From path of `*.contract`
    /// * From name of `*.contract`
    /// * From code_hash of `*.contract`
    pub fn rt(&mut self, contract: &str) -> crate::Result<Runtime> {
        if contract.is_empty() {
            return Err(crate::Error::Custom("Please provide an ink! contract"));
        }

        let if_path = PathBuf::from(contract);
        Ok(if if_path.exists() {
            let source = fs::read(if_path)?;
            let rt = Runtime::from_contract_and_storage(&source, self)?;
            self.0.insert(
                &rt.metadata.contract.name,
                bincode::serialize(&rt.metadata.clone())?,
            )?;
            rt
        } else if let Ok(Some(contract)) = self.0.get(contract.as_bytes()) {
            Runtime::from_metadata_and_storage(bincode::deserialize::<Metadata>(&contract)?, self)?
        } else {
            return Err(crate::Error::ParseContractFailed(contract.to_string()));
        })
    }
}

impl ceres_runtime::Storage for Storage {
    fn set(&mut self, code_hash: [u8; 32], data: BTreeMap<[u8; 32], Vec<u8>>) -> Result<()> {
        self.0
            .insert(
                &code_hash,
                bincode::serialize(&data).map_err(|_| Error::Custom {
                    err: "Serialize failed",
                })?,
            )
            .map_err(|_| Error::InsertContractFailed)?;
        Ok(())
    }

    fn get(&self, code_hash: [u8; 32]) -> Option<BTreeMap<[u8; 32], Vec<u8>>> {
        bincode::deserialize(&self.0.get(&code_hash).ok()??).ok()
    }

    fn new_state(&self) -> BTreeMap<[u8; 32], Vec<u8>> {
        BTreeMap::new()
    }
}
