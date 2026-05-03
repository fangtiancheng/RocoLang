use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_user_info", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_user_info().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("is_in_combat", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.is_in_combat().map_err(to_rhai_error)
        });
    }
}
