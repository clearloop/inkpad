//! WASM value

/// Value types
#[derive(Clone)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

/// Custom value
#[derive(Clone)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
}

/// Value for return
#[derive(Clone)]
pub enum ReturnValue {
    Unit,
    Value(Value),
}
