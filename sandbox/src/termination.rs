//! Termination Entry
use crate::Sandbox;
use ceres_executor::Result;

/// Termination Entry
pub struct TerminationEntry {
    pub beneficiary: [u8; 32],
}

impl Sandbox {
    pub fn terminate(&mut self, beneficiary: [u8; 32]) -> Result<u32> {
        self.terminations.push(TerminationEntry { beneficiary });
        Ok(0)
    }
}
