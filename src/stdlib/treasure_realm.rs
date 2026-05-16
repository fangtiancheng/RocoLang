use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1};
use crate::stdlib::RocoStdLib;

// Index convention:
// - buy/get_gift index is 1-based, matching AS curPage * 4 + index + 1.
// - boost_by_item index is 1-based: 1..3 for the three boost materials.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", treasure_realm_query);
    register_stdlib_fn_1!(module, stdlib, "buy", treasure_realm_buy, index: i64);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "boost_by_item",
        treasure_realm_boost_by_item,
        index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "boost_by_vip", treasure_realm_boost_by_vip);
    register_stdlib_fn_0!(module, stdlib, "start_battle", treasure_realm_start_battle);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_battle",
        treasure_realm_submit_battle
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "get_gift",
        treasure_realm_get_gift,
        index: i64
    );
}
