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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
}

impl Default for Value {
    fn default() -> Value {
        Value::F32(0)
    }
}

impl Value {
    /// try convert to i32
    pub fn as_i32(self) -> i32 {
        match self {
            Value::I32(v) => v,
            Value::I64(v) => v as i32,
            Value::F32(v) => v as i32,
            Value::F64(v) => v as i32,
        }
    }

    /// try convert to i64
    pub fn as_i64(self) -> i64 {
        match self {
            Value::I32(v) => v as i64,
            Value::I64(v) => v,
            Value::F32(v) => v as i64,
            Value::F64(v) => v as i64,
        }
    }

    /// try convert to u32
    pub fn as_u32(self) -> u32 {
        match self {
            Value::I32(v) => v as u32,
            Value::I64(v) => v as u32,
            Value::F32(v) => v,
            Value::F64(v) => v as u32,
        }
    }

    /// try convert to u64
    pub fn as_u64(self) -> u64 {
        match self {
            Value::I32(v) => v as u64,
            Value::I64(v) => v as u64,
            Value::F32(v) => v as u64,
            Value::F64(v) => v,
        }
    }
}

impl From<Value> for i32 {
    fn from(v: Value) -> i32 {
        v.as_i32()
    }
}

impl From<Value> for i64 {
    fn from(v: Value) -> i64 {
        v.as_i64()
    }
}

impl From<Value> for u32 {
    fn from(v: Value) -> u32 {
        v.as_u32()
    }
}

impl From<Value> for u64 {
    fn from(v: Value) -> u64 {
        v.as_u64()
    }
}
