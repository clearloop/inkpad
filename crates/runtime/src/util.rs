//! Memory generator
use crate::{Error, Result};
use ceres_std::Vec;
use parity_wasm::elements::{External, Module};

const IMPORT_MODULE_MEMORY: &str = "env";

/// Parse `Vec<String>` to `Vec<RuntimeValue>`
pub fn parse_args(selector: &str, args: &[&str], tys: Vec<u32>) -> Result<Vec<u8>> {
    println!("{:?}", tys.len());
    println!("{:?}", args.len());
    if args.len() != tys.len() {
        return Err(Error::InvalidArgumentLength);
    }

    let mut res = hex::decode(&selector[2..])
        .map_err(|_| Error::DecodeSelectorFailed)?
        .to_vec();
    for i in 0..args.len() {
        match args[i] {
            "true" => res.push(1),
            "false" => res.push(0),
            hex if hex.starts_with("0x") => {
                res.append(&mut hex::decode(&hex[2..]).map_err(|_| Error::ParseArgumentFailed)?)
            }
            patt => res.append(&mut hex::decode(&patt).map_err(|_| Error::ParseArgumentFailed)?),
        }
    }

    Ok(res)
}

/// Trim 0x prefix
pub fn step_hex(h: &str) -> Result<Vec<u8>> {
    if h.starts_with("0x") {
        hex::decode(&h[2..])
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
pub fn scan_imports<'m>(module: &Module) -> core::result::Result<(u32, Option<u32>), &'static str> {
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
