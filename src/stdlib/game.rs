use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, register_stdlib_fn_0, register_stdlib_fn_1, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "get_pause", get_pause);
    register_stdlib_fn_1!(module, stdlib, "set_pause", set_pause, enabled: bool);

    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_set_pause", move |enabled: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_set_pause(enabled).map_err(to_rhai_error)
        });
    }
}
