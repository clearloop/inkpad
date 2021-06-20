//! Memory generator
use crate::{Error, Result};
use ceres_std::Vec;
use parity_wasm::elements::{External, Module};

const IMPORT_MODULE_MEMORY: &str = "env";

/// Parse `Vec<u8>` to `Vec<RuntimeValue>`
pub fn parse_args(selector: &str, args: Vec<Vec<u8>>, tys: Vec<u32>) -> Result<Vec<u8>> {
    if args.len() != tys.len() {
        return Err(Error::InvalidArgumentLength {
            expect: tys.len(),
            input: args.len(),
        });
    }

    let mut res = step_hex(selector)
        .map_err(|_| Error::DecodeSelectorFailed)?
        .to_vec();
    for mut arg in args {
        res.append(&mut arg);
    }

    Ok(res)
}

/// Trim 0x prefix
pub fn step_hex(h: &str) -> Result<Vec<u8>> {
    if let Some(stripped) = h.strip_prefix("0x") {
        hex::decode(stripped)
    } else {
        hex::decode(&h)
    }
    .map_err(|_| Error::DecodeSelectorFailed)
}

/// Parse code hash from string
pub fn parse_code_hash(h: &str) -> Result<[u8; 32]> {
    let hash = step_hex(h)?;
    let mut res = [0; 32];
    if hash.len() != 32 {
        Err(Error::InvalidCodeHash)
    } else {
        res.copy_from_slice(&hash);
        Ok(res)
    }
}

/// Scan an import section if any.
///
/// This accomplishes two tasks:
///
/// - checks any imported function against defined host functions set, incl.
///   their signatures.
/// - if there is a memory import, returns it's descriptor
/// `import_fn_banlist`: list of function names that are disallowed to be imported
pub fn scan_imports(module: &Module) -> core::result::Result<(u32, Option<u32>), &'static str> {
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
                range = Some((limits.initial() as u32, limits.maximum().map(|v| v as u32)));
                continue;
            }
        };
    }

    if let Some(limit) = range {
        Ok(limit)
    } else {
        Ok((0, None))
    }
}
