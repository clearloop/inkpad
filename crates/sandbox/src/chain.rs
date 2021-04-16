//! Chain state
use crate::{util::al, Sandbox};
use ceres_std::Vec;
use parity_scale_codec::Encode;

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

    pub fn get_weight_price(&self, weight: u64) -> Vec<u8> {
        al(1312_u64.saturating_mul(weight).encode(), 16)
    }
}
