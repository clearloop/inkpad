use thiserror::Error;

/// Ceres Errors
#[derive(Error, Debug)]
pub enum E {
    #[error("Could not open `{0}`")]
    CouldNotOpenDatabase(String),
    #[error(transparent)]
    Sled(#[from] sled::Error),
    #[error(transparent)]
    ActicWeb(#[from] actix_web::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Subxt(#[from] substrate_subxt::Error),
    #[error(transparent)]
    Error(#[from] Box<dyn std::error::Error>),
}

/// Ceres Result
pub type Result<T> = std::result::Result<T, E>;
