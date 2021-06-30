//! Browser Result
use crate::ri::log;
use ceres_std::fmt::Debug;

/// Browser Error
#[derive(Debug)]
pub enum Error {
    /// RuntimeError
    Runtime { error: ceres_runtime::Error },
    /// Window not exists
    WindowNotExists,
    /// Could not find local storage
    LocalStorageNotExists,
    /// WebSys Error
    WebSysError,
    /// SerializeJson Error
    SerdeJson { error: serde_json::Error },
    /// Decode failed
    Hex { error: hex::FromHexError },
}

/// Browser Result
pub type Result<T> = core::result::Result<T, Error>;

///  check and panic error
pub fn err_check<T, E: Debug>(res: core::result::Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            log(&format!("{:?}", e));
            panic!("{:?}", e)
        }
    }
}
