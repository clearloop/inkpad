//! Browser Result
use crate::ri::log;
use ceres_std::fmt::Display;
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

///  check and panic error
pub fn err_check<T, E: Display>(res: core::result::Result<T, E>) -> T {
    match res {
        Ok(v) => v,
        Err(e) => {
            log(&e.to_string());
            panic!("{}", e)
        }
    }
}
