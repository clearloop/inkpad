//! Instantiate Entry
use crate::Sandbox;
use ceres_std::Vec;

/// Instantiate Entry
pub struct InstantiateEntry {
    pub code_hash: [u8; 32],
    pub endowment: u64,
    pub data: Vec<u8>,
    pub gas_left: u64,
    pub slat: Vec<u8>,
}

// impl Sandbox {
//     fn instantiate(&mut self, code_hash: [u8; 32], endowment: u64, gas_meter: ) {
//
//     }
// }
