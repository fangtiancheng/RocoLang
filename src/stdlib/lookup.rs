use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_array, to_rhai_error};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_item_info", move |item_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_item_info(item_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_strive_item_info", move |item_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_strive_item_info(item_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_strive_item_infos", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_strive_item_infos()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_guardian_pet_property_info", move |level: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_guardian_pet_property_info(level)
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_title_info", move |title_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_title_info(title_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_magic_info", move |magic_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_magic_info(magic_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_plugin_info", move |plugin_name: &str| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_plugin_info(plugin_name).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("list_plugin_infos", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.list_plugin_infos()
                .map(|plugins| to_array(&plugins))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_skill_info", move |skill_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_skill_info(skill_id).map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("lookup_spirit_info", move |spirit_id: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.lookup_spirit_info(spirit_id).map_err(to_rhai_error)
        });
    }
}
