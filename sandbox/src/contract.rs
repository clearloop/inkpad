//! Contract entry
use crate::Sandbox;
// use ceres_executor::Result;

impl Sandbox {
    pub fn tombstone_deposit(&self) -> u64 {
        16
    }

    pub fn rent_allowance(&self) -> u64 {
        self.rent_allowance
    }

    pub fn set_rent_allowance(&mut self, rent_allowence: u64) {
        self.rent_allowance = rent_allowence;
    }
}
