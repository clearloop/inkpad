//! Storage implementation
use ceres_executor::Memory;
use ceres_runtime::{Metadata, Runtime};
use ceres_support::{
    traits::{self, Cache, Frame},
    types::State,
};
use etc::{Etc, FileSystem, Meta};
use sled::{Db, Tree};
use std::{cell::RefCell, fs, path::PathBuf, process, rc::Rc};

const CACHE_STORAGE: &str = "cache";

/// A ceres storage implementation using sled
#[derive(Clone)]
pub struct Storage {
    pub db: Db,
    pub cache: Tree,
    frame: Vec<Rc<RefCell<State<Memory>>>>,
}

impl traits::Storage for Storage {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        let r = self.cache.insert(key, value).ok()?.map(|v| v.to_vec());
        self.cache.flush().ok();
        self.db.flush().ok();
        r
    }

    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let r = self.cache.remove(key).ok()?.map(|v| v.to_vec());
        self.cache.flush().ok();
        self.db.flush().ok();
        r
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.cache.get(key).ok()?.map(|v| v.to_vec())
    }
}

impl Cache<Memory> for Storage {
    fn frame(&self) -> &Vec<Rc<RefCell<State<Memory>>>> {
        &self.frame
    }

    fn frame_mut(&mut self) -> &mut Vec<Rc<RefCell<State<Memory>>>> {
        &mut self.frame
    }

    fn memory(&self) -> Option<Memory> {
        Some(self.frame.last()?.borrow().memory.clone())
    }
}

impl Frame<Memory> for Storage {}

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
        let etc = Etc::new(&dirs::home_dir().ok_or("Could not find home dir")?)?;
        let db = sled::open(etc.open(".ceres/contracts")?.real_path()?)?;
        let cache = db.open_tree(CACHE_STORAGE)?;

        Ok(Self {
            db,
            cache,
            frame: Vec::new(),
        })
    }

    /// Contract instance
    ///
    /// * From path of `*.contract`
    /// * From name of `*.contract`
    /// * From code_hash of `*.contract`
    pub fn rt(&mut self, contract: &str) -> crate::Result<Runtime> {
        let if_path = PathBuf::from(contract);
        let cache = self.clone();
        Ok(if if_path.exists() {
            let source = fs::read(if_path)?;
            let rt = Runtime::from_contract(&source, cache, Some(ceres_ri::Instance))?;
            self.db.insert(
                &rt.metadata.contract.name,
                bincode::serialize(&rt.metadata.clone())?,
            )?;
            rt
        } else if let Ok(Some(contract)) = if contract.is_empty() {
            let mut recent = None;
            for c in self.db.iter() {
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
            self.db.get(contract.as_bytes())
        } {
            Runtime::from_metadata(
                bincode::deserialize::<Metadata>(&contract)?,
                cache,
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
