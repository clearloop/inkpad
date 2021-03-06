//! Inkpad CLI Error
use inkpad_executor::Error as Executor;
use inkpad_runtime::Error as Runtime;
use inkpad_support::errors;
use etc::Error as Etc;
use parity_scale_codec::Error as Codec;
use sled::Error as Sled;
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as Io,
};

#[derive(Debug)]
pub struct CouldNotParseCommand(String);
#[derive(Debug)]
pub struct ParseContractFailed(String);
#[derive(Debug)]
pub struct DecodeAddressFailed(String);
#[derive(Debug)]
pub struct Custom(String);

errors! {
    Codec,
    Runtime,
    Executor,
    Etc,
    Sled,
    Io,
    CouldNotParseCommand,
    DecodeAddressFailed,
    ParseContractFailed,
    Custom
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Error {
        Error::Custom(s.into())
    }
}

/// Inkpad result
pub type Result<T> = core::result::Result<T, Error>;
