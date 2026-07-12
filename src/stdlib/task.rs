use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query_info_list", task_query_info_list);
    register_stdlib_fn_1!(module, stdlib, "accept", task_accept, task_id: i64);
    register_stdlib_fn_1!(module, stdlib, "complete", task_complete, task_id: i64);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "get_achievement_list",
        task_get_achievement_list
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "check_achievement_finish",
        task_check_achievement_finish,
        story_id: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "query_magic_growup_info",
        task_query_magic_growup_info
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "condition_query_status_raw",
        task_condition_query_status_raw,
        task_id: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "condition_apply_complete",
        task_condition_apply_complete,
        npc_id: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "condition_query_status",
        task_condition_query_status,
        task_id: i64
    );
}
