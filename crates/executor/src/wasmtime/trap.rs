//! Trap conversion
use crate::{
    trap::{Trap as OutterTrap, TrapCode as OutterTrapCode},
    Error,
};
use ceres_std::fmt;
use wasmtime::{Trap, TrapCode};

impl From<Trap> for Error {
    fn from(trap: Trap) -> Error {
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
            trace: format!("{:?}", trap)
                .split('\n')
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
