//! Implementation of wasmi return value
use crate::derive::{ReturnValue, Value};
use ::wasmi::RuntimeValue;

impl From<RuntimeValue> for Value {
    fn from(v: RuntimeValue) -> Value {
        match v {
            RuntimeValue::I32(v) => Value::I32(v),
            RuntimeValue::I64(v) => Value::I64(v),
            RuntimeValue::F32(v) => Value::F32(v.into()),
            RuntimeValue::F64(v) => Value::F64(v.into()),
        }
    }
}

impl From<Value> for RuntimeValue {
    fn from(v: Value) -> RuntimeValue {
        match v {
            Value::I32(v) => RuntimeValue::I32(v),
            Value::I64(v) => RuntimeValue::I64(v),
            Value::F32(v) => RuntimeValue::F32(v.into()),
            Value::F64(v) => RuntimeValue::F64(v.into()),
        }
    }
}

impl From<RuntimeValue> for ReturnValue {
    fn from(v: RuntimeValue) -> ReturnValue {
        match v {
            RuntimeValue::I32(v) => ReturnValue::Value(Value::I32(v)),
            RuntimeValue::I64(v) => ReturnValue::Value(Value::I64(v)),
            RuntimeValue::F32(v) => ReturnValue::Value(Value::F32(v.into())),
            RuntimeValue::F64(v) => ReturnValue::Value(Value::F64(v.into())),
        }
    }
}
