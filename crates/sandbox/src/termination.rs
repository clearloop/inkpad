//! Termination Entry
use crate::Sandbox;
use ceres_executor::Result;

/// Termination Entry
pub struct TerminationEntry {
    pub beneficiary: u32,
}

impl Sandbox {
    pub fn terminate(&mut self, beneficiary: u32) -> Result<u32> {
        self.ext.terminations.push(TerminationEntry { beneficiary });
        Ok(0)
    }
}
