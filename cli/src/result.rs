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
}

/// Ceres result
pub type Result<T> = core::result::Result<T, Error>;
