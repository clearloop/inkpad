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
