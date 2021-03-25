//! Memory generator
use parity_wasm::elements::{External, Module};

const IMPORT_MODULE_MEMORY: &str = "env";

/// Scan an import section if any.
///
/// This accomplishes two tasks:
///
/// - checks any imported function against defined host functions set, incl.
///   their signatures.
/// - if there is a memory import, returns it's descriptor
/// `import_fn_banlist`: list of function names that are disallowed to be imported
pub fn scan_imports<'m>(b: &[u8]) -> Result<Option<(u32, Option<u32>)>, &'static str> {
    let module = Module::from_bytes(b).map_err(|_| "Failed to parse module")?;

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
                range = Some((limits.initial(), limits.maximum()));
                continue;
            }
        };
    }

    Ok(range)
}
