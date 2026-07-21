use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - first_mix part_index: 0-based, matching ui1243 part_btn_0..5.
// - first_buy_item item_index: 0-based, matching ui1243 add_btn_0..3.
// - second game_index: 0-based, matching ui1244 state/m_curGame; 10 means one-click completion.
// - third_buy_score kind: 0=complete all score, 1=complete current task.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", taurus_first_query);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "first_get_leather",
        taurus_first_get_leather
    );
    register_stdlib_fn_0!(module, stdlib, "first_get_nail", taurus_first_get_nail);
    register_stdlib_fn_0!(module, stdlib, "first_get_ding", taurus_first_get_ding);
    register_stdlib_fn_0!(module, stdlib, "first_get_glue", taurus_first_get_glue);
    register_stdlib_fn_1!(module, stdlib, "first_mix", taurus_first_mix, part_index: i64);
    register_stdlib_fn_0!(module, stdlib, "first_buy_pet", taurus_first_buy_pet);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_buy_item",
        taurus_first_buy_item,
        item_index: i64,
        count: i64
    );

    register_stdlib_fn_0!(module, stdlib, "second_query", taurus_second_query);
    register_stdlib_fn_0!(module, stdlib, "second_query_bag", taurus_second_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_submit_game",
        taurus_second_submit_game,
        game_index: i64,
        score: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_buy_game",
        taurus_second_buy_game,
        game_index: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_evolve",
        taurus_second_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_buy_level",
        taurus_second_buy_level,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(module, stdlib, "third_query", taurus_third_query);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_settle_npc_fight",
        taurus_third_settle_npc_fight
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_evolve",
        taurus_third_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_buy_level",
        taurus_third_buy_level,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_buy_score",
        taurus_third_buy_score,
        kind: i64
    );
    register_stdlib_fn_0!(module, stdlib, "third_query_bag", taurus_third_query_bag);
    register_stdlib_fn_0!(module, stdlib, "third_get_task", taurus_third_get_task);
}
