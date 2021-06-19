//! Browser Result
use snafu::Snafu;

/// Browser Error
#[derive(Snafu, Debug)]
pub enum Error {
    /// RuntimeError
    #[snafu(display("runtime error {}", error))]
    Runtime { error: ceres_runtime::Error },
    /// Window not exists
    WindowNotExists,
    /// Could not find local storage
    LocalStorageNotExists,
    /// WebSys Error
    WebSysError,
    /// SerializeJson Error
    #[snafu(display("serde json error {}", error))]
    SerdeJson { error: serde_json::Error },
}

/// Browser Result
pub type Result<T> = core::result::Result<T, Error>;
