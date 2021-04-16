//! encode util
use ceres_std::{vec, Vec};

/// Fill bytes
pub fn al(mut b: Vec<u8>, at_least: usize) -> Vec<u8> {
    let len = b.len();
    let pad = at_least - len;
    if pad < 1 {
        b
    } else {
        let mut res = vec![0; pad];
        b.append(&mut res);
        b
    }
}

#[cfg(test)]
mod tests {
    use super::al;
    use parity_scale_codec::{Decode, Encode};

    #[test]
    fn test_balance_suffix_bytes() {
        let encoded = al(42_u32.encode(), 16);
        assert_eq!(encoded.len(), 16);
        assert_eq!(42, u32::decode(&mut encoded.as_slice()).unwrap());
    }
}
