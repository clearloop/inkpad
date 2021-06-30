//! Record the active frame
use crate::{Sandbox, StorageKey};
use ceres_executor::Result;

impl Sandbox {
    /// Active frame
    pub fn active(&mut self, frame: StorageKey) -> Result<()> {
        // self.flush_bucket()?;
        // self.stack.push(frame);
        // self.set_bucket()?;
        Ok(())
    }

    /// Exit frame
    pub fn exit(&mut self) -> Result<()> {
        // self.flush_bucket()?;
        // self.stack.pop();
        // self.set_bucket()?;
        Ok(())
    }
}
