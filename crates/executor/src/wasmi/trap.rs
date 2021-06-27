//! WASMi traps
use crate::{
    trap::{self, TrapCode},
    Error,
};
use ceres_std::{fmt, Box};
use wasmi::{Trap, TrapKind};

impl From<Trap> for trap::Trap {
    fn from(trap: Trap) -> trap::Trap {
        trap::Trap {
            code: match trap.kind() {
                TrapKind::StackOverflow => TrapCode::StackOverflow,
                TrapKind::DivisionByZero => TrapCode::IntegerDivisionByZero,
                TrapKind::ElemUninitialized => TrapCode::BadSignature,
                TrapKind::InvalidConversionToInt => TrapCode::BadConversionToInteger,
                TrapKind::MemoryAccessOutOfBounds => TrapCode::MemoryOutOfBounds,
                TrapKind::TableAccessOutOfBounds => TrapCode::TableOutOfBounds,
                TrapKind::UnexpectedSignature => TrapCode::BadSignature,
                TrapKind::Unreachable => TrapCode::UnreachableCodeReached,
                TrapKind::Host(e) => {
                    if let Some(e) = e.downcast_ref::<Error>() {
                        return TrapCode::HostError(Box::new(e.clone())).into();
                    } else {
                        TrapCode::Unknown
                    }
                }
            },
            trace: trap.wasm_trace().to_vec(),
        }
    }
}

impl fmt::Display for trap::Trap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let trace = &self.trace;
        if trace.is_empty() {
            write!(f, "[]")?;
        } else {
            for (index, trace) in trace.iter().enumerate() {
                if index == trace.len() - 1 {
                    write!(f, "\n\t╰─>")?;
                } else {
                    write!(f, "\n\t|  ")?;
                }
                write!(f, "{}", trace)?;
            }
        }

        Ok(())
    }
}
