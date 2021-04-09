//! Chain state
use crate::Sandbox;
// use ceres_executor::Result;
use ceres_std::Vec;
// use funty::AtLeast32;
// use parity_scale_codec::Encode;

impl Sandbox {
    pub fn deposit_event(&mut self, topics: Vec<[u8; 32]>, data: Vec<u8>) {
        self.events.push((topics, data));
    }

    pub fn block_number(&self) -> [u8; 32] {
        [0; 32]
    }

    pub fn max_value_size(&self) -> u32 {
        16_384
    }
}
