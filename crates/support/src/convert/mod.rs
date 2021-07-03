use crate::types::StorageKey;

/// Convert bytes to u32
pub fn to_u32(b: &[u8]) -> Option<u32> {
    if b.len() != 4 {
        None
    } else {
        let mut r = [0; 4];
        r.copy_from_slice(b);
        Some(u32::from_ne_bytes(r))
    }
}

/// Convert bytes tot storage key
pub fn to_storage_key(b: &[u8]) -> Option<StorageKey> {
    if b.len() != 32 {
        None
    } else {
        let mut r = [0; 32];
        r.copy_from_slice(b);
        Some(r)
    }
}

/// Trim 0x prefix
pub fn step_hex(h: &str) -> Option<Vec<u8>> {
    if let Some(stripped) = h.strip_prefix("0x") {
        hex::decode(stripped)
    } else {
        hex::decode(&h)
    }
    .ok()
}

/// Parse code hash from string
pub fn parse_code_hash(h: &str) -> Option<[u8; 32]> {
    let hash = step_hex(h)?;
    let mut res = [0; 32];
    if hash.len() != 32 {
        None
    } else {
        res.copy_from_slice(&hash);
        Some(res)
    }
}
