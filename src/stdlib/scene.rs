use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_array, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("move_to_scene", move |scene_id: i64, timeout_ms: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.move_to_scene(scene_id, timeout_ms)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "try_move_to_scene",
            move |scene_id: i64, timeout_ms: i64| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.try_move_to_scene(scene_id, timeout_ms)
                    .map_err(to_rhai_error)
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_current_scene", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_current_scene().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_scene_spirits", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_scene_spirits()
                .map(|spirits| to_array(&spirits))
                .map_err(to_rhai_error)
        });
    }
}
