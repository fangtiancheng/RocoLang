use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2, register_stdlib_fn_4,
};
use crate::stdlib::RocoStdLib;

// Index convention:
// - pool_version is 1-based, matching the AS/CGI `version` value.
// - wish_index is 1-based; use cancel_wish() instead of passing 0 directly.
// - item_index is 1-based within its exchange group.
// - exchange_kind: 0=magic, 1=props, 2=recycle.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", summon_query);
    register_stdlib_fn_0!(module, stdlib, "query_data", summon_query_data);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "set_wish",
        summon_set_wish,
        pool_version: i64,
        wish_index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "cancel_wish",
        summon_cancel_wish,
        pool_version: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "draw",
        summon_draw,
        pool_version: i64,
        draw_count: i64
    );
    register_stdlib_fn_4!(
        module,
        stdlib,
        "exchange",
        summon_exchange,
        exchange_kind: i64,
        pool_version: i64,
        item_index: i64,
        count: i64
    );
    register_stdlib_fn_0!(module, stdlib, "query_record", summon_query_record);
}
