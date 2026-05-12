use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    lock_stdlib, register_stdlib_fn_1, register_stdlib_fn_2, to_array, to_rhai_error,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_ground_info", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_get_ground_info().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_seed_bag", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_get_seed_bag()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_2!(module, stdlib, "sow", manor_sow, seed_id: i64, ground_id: i64);
    register_stdlib_fn_1!(module, stdlib, "reap", manor_reap, ground_id: i64);
    register_stdlib_fn_1!(module, stdlib, "uproot", manor_uproot, ground_id: i64);
    register_stdlib_fn_2!(module, stdlib, "weed", manor_weed, ground_id: i64, weed_type: i64);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "use_fertilizer",
        manor_use_fertilizer,
        ground_id: i64,
        fertilizer_item_id: i64
    );
}
