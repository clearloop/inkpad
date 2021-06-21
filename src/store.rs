//! Storage implementation
use ceres_runtime::{Error, Metadata, Runtime};
use ceres_support::{traits, types::StorageKey};
use etc::{Etc, FileSystem, Meta};
use sled::Db;
use std::{cell::RefCell, fs, path::PathBuf, process, rc::Rc};

const CERES_CACHE_TREE: &str = "CERES_CACHE_TREE";
const CERES_STATE_TREE: &str = "CERES_STATE_TREE";

/// Custom Tree
pub struct Tree(pub sled::Tree);

impl traits::Storage for Tree {
    fn get(&self, code_hash: [u8; 32]) -> Option<Vec<u8>> {
        bincode::deserialize(&self.0.get(&code_hash).ok()??).ok()
    }

    fn set(&mut self, code_hash: StorageKey, data: Vec<u8>) -> Option<StorageKey> {
        self.0
            .insert(
                &code_hash,
                bincode::serialize(&data)
                    .map_err(|_| Error::Custom {
                        err: "Serialize failed",
                    })
                    .ok()?,
            )
            .ok()
            .map(|_| code_hash)
    }
}

/// A ceres storage implementation using sled
#[derive(Clone)]
pub struct Storage(pub Db);

impl Storage {
    fn quit() {
        println!(
            "The following required arguments were not provided: \n\t\t\
             <*.contract | name | code-hash>"
        );
        process::exit(1);
    }

    /// New storage
    pub fn new() -> crate::Result<Self> {
        let etc =
            Etc::new(&dirs::home_dir().ok_or(crate::Error::Custom("Could not find home dir"))?)?;

        Ok(Self(sled::open(
            etc.open(".ceres/contracts")?.real_path()?,
        )?))
    }

    /// Contract instance
    ///
    /// * From path of `*.contract`
    /// * From name of `*.contract`
    /// * From code_hash of `*.contract`
    pub fn rt(&mut self, contract: &str) -> crate::Result<Runtime> {
        let if_path = PathBuf::from(contract);
        let cache = Rc::new(RefCell::new(Tree(self.0.open_tree(CERES_CACHE_TREE)?)));
        let state = Rc::new(RefCell::new(Tree(self.0.open_tree(CERES_STATE_TREE)?)));
        Ok(if if_path.exists() {
            let source = fs::read(if_path)?;
            let rt = Runtime::from_contract_and_storage(
                &source,
                cache,
                state,
                Some(ceres_ri::Instance),
            )?;
            self.0.insert(
                &rt.metadata.contract.name,
                bincode::serialize(&rt.metadata.clone())?,
            )?;
            rt
        } else if let Ok(Some(contract)) = if contract.is_empty() {
            let mut recent = None;
            for c in self.0.iter() {
                let (k, v) = c?;
                if k.len() != 32 {
                    recent = Some(Ok(Some(v)));
                    break;
                }
            }

            if let Some(r) = recent {
                r
            } else {
                return Err(crate::Error::ParseContractFailed(
                    "Get recent contract failed".to_string(),
                ));
            }
        } else {
            self.0.get(contract.as_bytes())
        } {
            Runtime::from_metadata_and_storage(
                bincode::deserialize::<Metadata>(&contract)?,
                cache,
                state,
                Some(ceres_ri::Instance),
            )?
        } else {
            Self::quit();

            // NOTE:
            //
            // Unreachable error
            return Err(crate::Error::ParseContractFailed(contract.to_string()));
        })
    }
}
