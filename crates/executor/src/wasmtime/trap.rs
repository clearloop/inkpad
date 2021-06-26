//! Trap conversion
use crate::{
    trap::{Trap as OutterTrap, TrapCode as OutterTrapCode},
    Error, ReturnData,
};
use ::wasmtime::{Trap, TrapCode};
use ceres_std::fmt;
use parity_scale_codec::Decode;

impl From<Trap> for Error {
    fn from(trap: Trap) -> Error {
        let fmt = format!("{}", trap);
        if let Some(Ok(Ok(data))) = fmt
            .strip_prefix("0x")
            .map(|s| hex::decode(s).map(|ret| ReturnData::decode(&mut ret.as_ref())))
        {
            return Error::Return(data);
        }

        // parse code
        let mut code = OutterTrapCode::Unknown;
        if let Some(cc) = trap.trap_code() {
            code = match cc {
                TrapCode::BadConversionToInteger => OutterTrapCode::BadConversionToInteger,
                TrapCode::BadSignature => OutterTrapCode::BadSignature,
                TrapCode::HeapMisaligned => OutterTrapCode::HeapMisaligned,
                TrapCode::IndirectCallToNull => OutterTrapCode::IndirectCallToNull,
                TrapCode::IntegerDivisionByZero => OutterTrapCode::IntegerDivisionByZero,
                TrapCode::IntegerOverflow => OutterTrapCode::IntegerOverflow,
                TrapCode::Interrupt => OutterTrapCode::Interrupt,
                TrapCode::MemoryOutOfBounds => OutterTrapCode::MemoryOutOfBounds,
                TrapCode::StackOverflow => OutterTrapCode::StackOverflow,
                TrapCode::TableOutOfBounds => OutterTrapCode::TableOutOfBounds,
                TrapCode::UnreachableCodeReached => OutterTrapCode::UnreachableCodeReached,
                _ => OutterTrapCode::Unknown,
            }
        }

        Error::Trap(OutterTrap {
            code,
            trace: fmt
                .split('\n')
                .filter(|s| !s.is_empty() && !s.contains("<unknown>"))
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        })
    }
}

impl fmt::Display for OutterTrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.trace
            .iter()
            .map(|s| write!(f, "{}", s))
            .collect::<Result<Vec<_>, fmt::Error>>()?;
        Ok(())
    }
}
