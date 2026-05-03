use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{lock_stdlib, to_rhai_error};
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
