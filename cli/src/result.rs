use thiserror::Error;

/// Ceres CLI Error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse command `{0}`")]
    CouldNotParseCommand(String),
    #[error("Decode ss58 address `{0}`")]
    DecodeAddressFailed(String),
    #[error("FileSystem Error")]
    FileSystemError(#[from] etc::Error),
    #[error("Sled Error")]
    SledError(#[from] sled::Error),
    #[error("`{0}`")]
    Curstom(&'static str),
    #[error("`{0}`")]
    IoError(#[from] std::io::Error),
    #[error("`{0}`")]
    RuntimeError(#[from] ceres_runtime::Error),
    #[error("`{0}`")]
    ParseContractFailed(String),
    #[error("`{0}`")]
    SerializeFailed(#[from] bincode::Error),
}

/// Ceres result
pub type Result<T> = core::result::Result<T, Error>;
