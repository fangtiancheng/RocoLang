use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("sleep", move |ms: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.sleep(ms).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("now_ms", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.now_ms().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("sleep_until_ms", move |target_ms: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.sleep_until_ms(target_ms).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("format_time", move |timestamp: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.format_time(timestamp).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("log", move |message: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.log(message).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("assert", move |condition: bool, message: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.assert(condition, message).map_err(to_rhai_error)
        });
    }
}
