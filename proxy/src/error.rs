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
}

/// Ceres Result
pub type Result<T> = std::result::Result<T, E>;
