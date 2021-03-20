//! Shared data
use crate::{EuropaRuntime, Result, E};
use sled::Db;
use substrate_subxt::{Client, ClientBuilder};

/// The data root of ceres
const DATA_ROOT: &str = "io.patract.ceres";

/// Shared data
pub struct Share {
    /// Databse
    pub sled: Db,
    /// API Client
    ///
    /// With default port 4242
    pub client: Client<EuropaRuntime>,
}

impl Share {
    /// New share
    ///
    /// * Create database
    pub async fn new(endpoint: &str) -> Result<Share> {
        Ok(Share {
            sled: sled::open(
                dirs::data_dir()
                    .ok_or(E::CouldNotOpenDatabase("datadir".into()))?
                    .join(DATA_ROOT)
                    .join("db"),
            )?,
            client: ClientBuilder::<EuropaRuntime>::new()
                .set_url(endpoint)
                .build()
                .await?,
        })
    }
}
