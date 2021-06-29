#![cfg_attr(not(feature = "std"), no_std)]
mod builder;
mod executor;
mod func;
mod instance;
mod memory;
mod result;
mod trap;
mod value;

pub mod derive;

#[cfg(not(feature = "std"))]
mod wasmi;
#[cfg(feature = "std")]
mod wasmtime;

pub use self::{
    builder::Builder,
    executor::Executor,
    instance::Instance,
    memory::Memory,
    result::{Error, ExecResult, Result, ReturnCode, ReturnData, ReturnFlags},
    trap::{Trap, TrapCode},
    value::Value,
};
