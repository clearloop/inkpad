//! Storage implementation
use ceres_executor::Memory;
use ceres_runtime::{Metadata, Runtime};
use ceres_support::{
    traits::{self, Cache, Frame},
    types::State,
};
use etc::{Etc, FileSystem, Meta};
use sled::Db;
use std::{cell::RefCell, fs, path::PathBuf, process, rc::Rc};

/// A ceres storage implementation using sled
#[derive(Clone)]
pub struct Storage {
    pub db: Db,
    frame: Vec<Rc<RefCell<State<Memory>>>>,
}

impl traits::Storage for Storage {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.db.insert(key, value).ok()?.map(|v| v.to_vec())
    }

    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.remove(key).ok()?.map(|v| v.to_vec())
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key).ok()?.map(|v| v.to_vec())
    }
}

impl Frame<Memory> for Storage {
    fn active(&self) -> Option<[u8; 32]> {
        Some(self.frame.last()?.borrow().hash)
    }

    fn active_set(&self, key: [u8; 32], value: Vec<u8>) -> Option<Vec<u8>> {
        self.frame
            .last()
            .map(|state| state.borrow_mut().set(key.to_vec(), value))?
    }

    fn active_get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.frame
            .last()
            .map(|state| state.borrow().get(key).map(|v| v.to_vec()))?
    }

    fn push(&mut self, code_hash: [u8; 32], memory: Memory) {
        self.frame
            .push(Rc::new(RefCell::new(State::new(code_hash, memory))));
    }

    #[allow(mutable_borrow_reservation_conflict)]
    fn switch(&mut self, code_hash: [u8; 32]) -> Option<()> {
        for frame in &self.frame {
            if frame.borrow().hash != code_hash {
                continue;
            }

            self.frame.push(frame.clone());
            return Some(());
        }

        None
    }

    fn back(&mut self) -> Option<()> {
        if self.frame.len() < 2 {
            None
        } else {
            self.frame.push(self.frame[self.frame.len() - 1].clone());
            Some(())
        }
    }

    fn top(&mut self) -> Option<()> {
        self.frame.push(self.frame[0].clone());
        Some(())
    }

    fn memory(&self) -> Option<Memory> {
        Some(self.frame.last()?.borrow().memory.clone())
    }
}

impl Cache<Memory> for Storage {}

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

        Ok(Self {
            db: sled::open(etc.open(".ceres/contracts")?.real_path()?)?,
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
