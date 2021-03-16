//! Shared data
use crate::{Result, E};
use sled::Db;

/// The data root of ceres
const DATA_ROOT: &str = "io.patract.ceres";

/// Shared data
pub struct Share {
    /// Databse
    pub sled: Db,
}

impl Share {
    /// New share
    ///
    /// * Create database
    pub fn new() -> Result<Share> {
        Ok(Share {
            sled: sled::open(
                dirs::data_dir()
                    .ok_or(E::CouldNotOpenDatabase("datadir".into()))?
                    .join(DATA_ROOT)
                    .join("db"),
            )?,
        })
    }
}
