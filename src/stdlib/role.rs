use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_array, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_cached_scene_roles", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_cached_scene_roles()
                .map(|roles| to_array(&roles))
                .map_err(to_rhai_error)
        });
    }
}
