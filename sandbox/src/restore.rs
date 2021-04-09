//! Restore Entry
use crate::Sandbox;
use ceres_executor::Result;
use ceres_std::Vec;

/// Restore Entry
pub struct RestoreEntry {
    pub dest: [u8; 32],
    pub code_hash: [u8; 32],
    pub rent_allowance: u64,
    pub delta: Vec<[u8; 32]>,
}

impl Sandbox {
    pub fn restore_to(
        &mut self,
        dest: [u8; 32],
        code_hash: [u8; 32],
        rent_allowance: u64,
        delta: Vec<[u8; 32]>,
    ) -> Result<(u32, u32)> {
        self.restores.push(RestoreEntry {
            dest,
            code_hash,
            rent_allowance,
            delta,
        });
        Ok((0, 0))
    }
}
