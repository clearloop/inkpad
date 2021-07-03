//! Storage implementation
use crate::Result;
use ceres_executor::Memory;
use ceres_runtime::Runtime;
use ceres_std::BTreeMap;
use ceres_support::{
    traits::{self, Cache, Frame},
    types::{Metadata, State},
};
use etc::{Etc, FileSystem, Meta};
use parity_scale_codec::{Decode, Encode};
use sled::{Db, Tree};
use std::{cell::RefCell, fs, path::PathBuf, process, rc::Rc};

const RUNTIME_CACHE: &str = "RUNTIME_CACHE";
const PREVIOUS_STATE: &str = "PREVIOUS_STATE";
type HostState = BTreeMap<[u8; 32], BTreeMap<Vec<u8>, Vec<u8>>>;

/// A ceres storage implementation using sled
#[derive(Clone)]
pub struct Storage {
    pub db: Db,
    cache: Tree,
    frame: Vec<Rc<RefCell<State<Memory>>>>,
    state: HostState,
}

impl traits::Storage for Storage {
    fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<Vec<u8>> {
        self.cache.insert(key, value).ok()?.map(|v| v.to_vec())
    }

    fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.cache.remove(key).ok()?.map(|v| v.to_vec())
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

    /// Flush data
    fn flush(&mut self) -> Option<()> {
        for state in self.frame.iter() {
            let state = state.borrow().clone();
            self.state.insert(state.hash, state.state);
        }

        let mut data = if let Some(state) = self.db.get(PREVIOUS_STATE).ok()? {
            HostState::decode(&mut state.as_ref()).ok()?
        } else {
            BTreeMap::new()
        };

        data.append(&mut self.state.clone());
        self.db.insert(PREVIOUS_STATE, data.encode()).ok()?;
        self.db.flush().ok()?;
        Some(())
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
        let cache = db.open_tree(RUNTIME_CACHE)?;

        Ok(Self {
            db,
            cache,
            frame: Vec::new(),
            state: BTreeMap::new(),
        })
    }

    fn load(&mut self, rt: &mut Runtime) -> Result<()> {
        let previous = self.db.get(PREVIOUS_STATE)?;
        if previous.is_none() {
            return Ok(());
        }

        for (code_hash, map) in
            HostState::decode(&mut previous.ok_or("Get previous data failed")?.as_ref())?
                .into_iter()
        {
            log::info!("loaded contract: 0x{}", hex::encode(code_hash));
            rt.sandbox.prepare(code_hash)?;
            rt.sandbox
                .cache
                .borrow_mut()
                .frame_mut()
                .last_mut()
                .ok_or("Could not get last frame")?
                .borrow_mut()
                .state = map;
        }

        let mut cache = rt.sandbox.cache.borrow_mut();
        let first = cache
            .frame()
            .first()
            .ok_or("No frame in current runtime")?
            .clone();
        cache.frame_mut().push(first.clone());
        Ok(())
    }

    /// Contract instance
    ///
    /// * From path of `*.contract`
    /// * From name of `*.contract`
    /// * From code_hash of `*.contract`
    pub fn rt(&mut self, contract: &str) -> crate::Result<Runtime> {
        let if_path = PathBuf::from(contract);
        let cache = self.clone();
        let mut runtime = if if_path.exists() {
            // init from source
            let source = fs::read(contract)?;
            let r = Runtime::from_contract(&source, cache, Some(ceres_ri::Instance))?;
            self.db.insert(
                r.cache
                    .borrow()
                    .active()
                    .ok_or(ceres_executor::Error::CodeNotFound)?
                    .clone(),
                r.metadata.encode(),
            )?;
            r
        } else if let Ok(Some(contract)) = if contract.is_empty() {
            // init from recent
            let mut recent = None;
            for c in self.db.iter() {
                let (k, v) = c?;
                if k.len() == 32 {
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
                Metadata::decode(&mut contract.as_ref())?,
                cache,
                Some(ceres_ri::Instance),
            )?
        } else {
            Self::quit();

            // NOTE:
            //
            // Unreachable error
            return Err(crate::Error::ParseContractFailed(contract.to_string()));
        };

        // load previous data
        self.load(&mut runtime)?;

        // flush data
        self.flush().ok_or("Flush data failed")?;

        // returns rt
        Ok(runtime)
    }
}
