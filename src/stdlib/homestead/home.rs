use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    lock_stdlib, register_stdlib_fn_1, register_stdlib_fn_2, to_array, to_rhai_error,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_1!(module, stdlib, "get_overview", home_get_overview, area_id: i64);
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_friend_list", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.home_get_friend_list()
                .map(|friends| to_array(&friends))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_training_spirits", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.home_get_training_spirits()
                .map(|spirits| to_array(&spirits))
                .map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_2!(
        module,
        stdlib,
        "get_training_spirit_report",
        home_get_training_spirit_report,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "take_training_spirit",
        home_take_training_spirit,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "query_coach_spirits",
        home_query_coach_spirits,
        refresh: bool
    );
}
