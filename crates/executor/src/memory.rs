//! Ceres executor memory
#[cfg(not(feature = "std"))]
use crate::wasmi as e;
#[cfg(feature = "std")]
use crate::wasmtime as e;
use crate::{derive, Result};
use core::ops;
use parity_wasm::elements::{External, Module};

const IMPORT_MODULE_MEMORY: &str = "env";

/// WASM executor liner memory
#[derive(Clone)]
pub struct Memory(pub e::Memory);

impl ops::Deref for Memory {
    type Target = e::Memory;

    fn deref(&self) -> &e::Memory {
        &self.0
    }
}

impl Memory {
    /// New liner memory
    pub fn new(initial: u32, maximum: Option<u32>) -> Result<Self> {
        Ok(Self(<e::Memory as derive::Memory>::new(initial, maximum)?))
    }

    /// Read a memory area at the address `ptr` with the size of the provided slice `buf`.
    pub fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<()> {
        derive::Memory::get(&self.0, ptr, buf)
    }

    /// Write a memory area at the address `ptr` with contents of the provided slice `buf`.
    pub fn set(&self, ptr: u32, value: &[u8]) -> Result<()> {
        derive::Memory::set(&self.0, ptr, value)
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
