//! Util
use crate::{Error, Result};

/// Decode address to [u8; 32]
///
/// ```
/// assert!(
///   ceres_cli::util::decode_addr(
///     "0x46da65a1be5b49d639a934e27b8a773c3fc2540f488df4c2afb9880ee34a6346"
///   ).is_ok()
/// );
/// ```
pub fn decode_addr(addr: &str) -> Result<[u8; 32]> {
    let mut slice = hex::decode(if addr.starts_with("0x") {
        &addr[2..]
    } else {
        &addr[..]
    })
    .map_err(|_| Error::DecodeAddressFailed(addr.into()))?;
    if slice.len() != 32 {
        return Err(Error::DecodeAddressFailed(addr.into()));
    }

    let mut res: [u8; 32] = [0; 32];
    res.copy_from_slice(&mut slice);
    Ok(res)
}

/// Pad spaces for str
pub fn pad(s: &str, spaces: usize) -> String {
    let pad = spaces - s.len();
    let mut o = String::new();
    o.push_str(&s);
    o.push_str(&" ".repeat(pad));
    o
}
