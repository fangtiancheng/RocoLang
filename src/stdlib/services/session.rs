use std::sync::{Arc, Mutex};

use rhai::{ImmutableString, Map, Module};

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_get_int", move |key: &str, default_value: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.session_get_int(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_set_int", move |key: &str, value: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.session_set_int(key, value).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn(
            "session_get_string",
            move |key: &str, default_value: &str| {
                let mut lib = lock_stdlib(&stdlib)?;
                lib.session_get_string(key, default_value)
                    .map_err(to_rhai_error)
            },
        );
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_set_string", move |key: &str, value: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.session_set_string(key, value).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_get_bool", move |key: &str, default_value: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.session_get_bool(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_set_bool", move |key: &str, value: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.session_set_bool(key, value).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_delete", move |key: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.session_delete(key).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_clear", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.session_clear().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("session_list_keys", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            let keys = lib.session_list_keys().map_err(to_rhai_error)?;
            let mut map = Map::new();
            for (key, value_type) in keys {
                map.insert(
                    ImmutableString::from(key).into(),
                    ImmutableString::from(value_type).into(),
                );
            }
            Ok(map)
        });
    }
}
