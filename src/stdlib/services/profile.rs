use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_rhai_error_in_context};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_user_info", move |context: rhai::NativeCallContext| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.get_user_info()
                .map_err(|error| to_rhai_error_in_context(error, &context))
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("is_in_combat", move |context: rhai::NativeCallContext| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.is_in_combat()
                .map_err(|error| to_rhai_error_in_context(error, &context))
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "query_server_time",
            move |context: rhai::NativeCallContext| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.query_server_time()
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "try_query_server_time",
            move |context: rhai::NativeCallContext| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.try_query_server_time()
                    .map_err(|error| to_rhai_error_in_context(error, &context))
            },
        );
    }
}
