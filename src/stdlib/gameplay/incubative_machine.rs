use crate::stdlib::util::{
    register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2, register_stdlib_fn_5,
};
use crate::stdlib::RocoStdLib;
use rhai::Module;
use std::sync::{Arc, Mutex};

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_2!(module, stdlib, "query_egg_list", incubative_machine_query_egg_list, egg_type: i64, page_index: i64);
    register_stdlib_fn_1!(module, stdlib, "query_info", incubative_machine_query_info, which: i64);
    register_stdlib_fn_5!(module, stdlib, "begin", incubative_machine_begin, which: i64, egg_type: i64, egg_id: i64, catch_time: i64, egg_uin: i64);
    register_stdlib_fn_1!(module, stdlib, "vip_speed_up", incubative_machine_vip_speed_up, which: i64);
    register_stdlib_fn_1!(module, stdlib, "teach", incubative_machine_teach, which: i64);
    register_stdlib_fn_1!(module, stdlib, "terminate", incubative_machine_terminate, which: i64);
    register_stdlib_fn_1!(module, stdlib, "get_spirit", incubative_machine_get_spirit, which: i64);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "set_complete_guide",
        incubative_machine_set_complete_guide
    );
}
