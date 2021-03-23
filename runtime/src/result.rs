//! Custom result
use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum Error {
    /// Memory out of bounds
    OutOfBounds,
}

pub type Result<T> = core::result::Result<T, Error>;
