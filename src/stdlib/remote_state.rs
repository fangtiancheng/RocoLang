use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::{
    util::{lock_stdlib, to_rhai_error_in_context},
    RocoStdLib,
};

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "load_scene_data",
            move |context: rhai::NativeCallContext, scene_id: i64| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.load_remote_scene_data(scene_id)
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "update_scene_data",
            move |context: rhai::NativeCallContext, scene_id: i64, values: rhai::Blob| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.update_remote_scene_data(scene_id, values)
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "load_npc_value",
            move |context: rhai::NativeCallContext, npc_id: i64| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.load_remote_npc_value(npc_id)
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        module.set_native_fn(
            "update_npc_value",
            move |context: rhai::NativeCallContext, npc_id: i64, value: i64| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.update_remote_npc_value(npc_id, value)
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
}
