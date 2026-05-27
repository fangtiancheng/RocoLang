use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", ice_crystal_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "charge_with_item",
        ice_crystal_charge_with_item,
        item: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "report_battle_win",
        ice_crystal_report_battle_win
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "condense_crystal",
        ice_crystal_condense_crystal,
        crystal: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "query_evolution_bag",
        ice_crystal_query_evolution_bag,
        target: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "evolve",
        ice_crystal_evolve,
        target: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "one_key_evolve",
        ice_crystal_one_key_evolve,
        target: i64,
        catch_time: i64
    );
}
