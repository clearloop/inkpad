//! wasm value

/// Value types
pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

/// Custom value
pub enum Value {
    I32(i32),
    I64(u64),
    F32(u32),
    F64(u64),
}

/// Value for return
pub enum ReturnValue {
    Value(Value),
}
