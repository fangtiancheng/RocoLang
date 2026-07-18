use std::sync::{Arc, Mutex};

use rhai::{ImmutableString, Map, Module};

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("today", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_today().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_get_int", move |key: &str, default_value: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_get_int(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_set_int", move |key: &str, value: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_set_int(key, value).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_increment_int", move |key: &str, delta: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_increment_int(key, delta)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_get_string", move |key: &str, default_value: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_get_string(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_set_string", move |key: &str, value: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_set_string(key, value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_get_bool", move |key: &str, default_value: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_get_bool(key, default_value)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_set_bool", move |key: &str, value: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_set_bool(key, value).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_delete", move |key: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_delete(key).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_clear", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_clear().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_list_keys", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            let keys = lib.memory_daily_list_keys().map_err(to_rhai_error)?;
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
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_battle_observed_started", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_battle_observed_started()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_battle_observed_completed", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_battle_observed_completed()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_battle_tracking_since", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_battle_tracking_since()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_battle_limit_reached", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_battle_limit_reached()
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_battle_limit", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_battle_limit().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("daily_battle_limit_return_code", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_battle_limit_return_code()
                .map_err(to_rhai_error)
        });
    }
    {
        module.set_native_fn("daily_battle_limit_message", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.memory_daily_battle_limit_message()
                .map_err(to_rhai_error)
        });
    }
}
