//! Storage implementation
use ceres_runtime::{Error, Result};
use ceres_std::BTreeMap;
use etc::{Etc, FileSystem, Meta};
use sled::Db;

/// A ceres storage implementation using sled
pub struct Storage(Db);

impl Storage {
    /// New storage
    pub fn new() -> crate::Result<Self> {
        let etc =
            Etc::new(&dirs::home_dir().ok_or(crate::Error::Curstom("Could not find home dir"))?)?;
        etc.mkdir(".ceres")?;

        Ok(Storage(sled::open(etc.real_path()?)?))
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
