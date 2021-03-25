//! Memory generator
use crate::{Error, Result};
use parity_wasm::elements::{External, Module};
use wasmi::memory_units::Pages;

const IMPORT_MODULE_MEMORY: &str = "env";

/// Trim 0x prefix
pub fn step_hex(h: &str) -> Result<Vec<u8>> {
    if h.starts_with("0x") {
        hex::decode(&h[2..])
    } else {
        hex::decode(&h)
    }
    .map_err(|_| Error::DecodeSelectorFailed)
}

/// Scan an import section if any.
///
/// This accomplishes two tasks:
///
/// - checks any imported function against defined host functions set, incl.
///   their signatures.
/// - if there is a memory import, returns it's descriptor
/// `import_fn_banlist`: list of function names that are disallowed to be imported
pub fn scan_imports<'m>(
    module: &Module,
) -> core::result::Result<(Pages, Option<Pages>), &'static str> {
    let import_entries = module
        .import_section()
        .map(|is| is.entries())
        .unwrap_or(&[]);

    let mut range = None;
    for import in import_entries {
        match import.external() {
            External::Table(_) => return Err("Cannot import tables"),
            External::Global(_) => return Err("Cannot import globals"),
            External::Function(ref type_idx) => type_idx,
            External::Memory(ref memory_type) => {
                if import.module() != IMPORT_MODULE_MEMORY {
                    return Err("Invalid module for imported memory");
                }
                if import.field() != "memory" {
                    return Err("Memory import must have the field name 'memory'");
                }
                if range.is_some() {
                    return Err("Multiple memory imports defined");
                }

                let limits = memory_type.limits();
                range = Some((
                    Pages(limits.initial() as usize),
                    limits.maximum().map(|v| Pages(v as usize)),
                ));
                continue;
            }
        };
    }

    if let Some(limit) = range {
        Ok(limit)
    } else {
        Ok((Pages(0 as usize), None))
    }
}
