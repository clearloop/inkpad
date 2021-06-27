//! Util
use crate::{
    derive::{HostFuncType, Value},
    Error,
};
use ceres_std::vec;
use core::mem;
use parity_scale_codec::Encode;
use wasmtime::{
    Caller,
    Config,
    Engine,
    Func,
    FuncType,
    Store,
    Trap,
    Val, // WasmBacktraceDetails,
};

/// Create store with DWARF enabled
///
/// NOTE: The Debug info with native trace has some problem in
/// aarch64-apple-darwin, not enable for default for now.
pub fn store_with_dwarf() -> Result<Store, Error> {
    Ok(Store::new(
        &Engine::new(
		&Config::new()
			// .debug_info(true)
			// .wasm_backtrace_details(WasmBacktraceDetails::Enable),
	)
        .map_err(|_| Error::CreateWasmtimeConfigFailed)?,
    ))
}

/// Wrap host function into `Func`
pub fn wrap_fn<T>(store: &Store, state: usize, f: usize, sig: FuncType) -> Func {
    let func = move |_: Caller<'_>, args: &[Val], results: &mut [Val]| {
        let mut inner_args = vec![];
        for arg in args {
            inner_args.push(from_val(arg.clone()));
        }

        // HACK the LIFETIME
        //
        // # Safety
        //
        // Work for one call.
        let state: &mut T = unsafe { mem::transmute(state) };
        let func: HostFuncType<T> = unsafe { mem::transmute(f) };
        match func(state, &inner_args) {
            Ok(ret) => {
                // # Safty
                //
                // This `result.len()` should always <= 1 since the length of
                // the result of `HostFuncType` is 1
                if results.len() == 1 {
                    results[0] = to_val(ret);
                } else if results.len() > 1 {
                    return Err(anyhow::Error::new(Error::UnExpectedReturnValue).into());
                }
                Ok(())
            }
            Err(e) => Err(match e {
                Error::Return(data) => Trap::new(format!("0x{}", hex::encode(data.encode()))),
                e => anyhow::Error::new(e).into(),
            }),
        }
    };
    Func::new(store, sig, func)
}

pub fn from_val(v: Val) -> Value {
    match v {
        Val::F32(v) => Value::F32(v),
        Val::I32(v) => Value::I32(v),
        Val::F64(v) => Value::F64(v),
        Val::I64(v) => Value::I64(v),
        _ => Value::F32(0),
    }
}

pub fn to_val(v: Value) -> Val {
    match v {
        Value::F32(v) => Val::F32(v),
        Value::F64(v) => Val::F64(v),
        Value::I32(v) => Val::I32(v),
        Value::I64(v) => Val::I64(v),
    }
}
