//! WASMi traps
use crate::trap::{self, TrapCode};
use ceres_std::fmt;
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
                TrapKind::Host(_) => TrapCode::HostError,
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
