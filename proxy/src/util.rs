// use crate::Result;
// use contract_metadata::ContractMetadata;
// use std::{fs, path::Path};

/// Convert string to snake case for storage key
pub fn sc(src: String) -> String {
    src.split_whitespace()
        .collect::<Vec<&str>>()
        .join("_")
        .to_ascii_lowercase()
}

// /// Metadada from `.contract` file
// pub fn mc<P>(p: P) -> Result<ContractMetadata>
// where
//     P: AsRef<Path>,
// {
//     Ok(serde_json::from_str(&fs::read_to_string(p)?)?)
// }
