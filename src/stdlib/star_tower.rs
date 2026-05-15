use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2, register_stdlib_fn_3,
};
use crate::stdlib::RocoStdLib;

// Index convention:
// - Star tower storey_index/node_index/reward_index are 0-based, matching the
//   CGI and the StarTowerInfo fields returned by query().
// - Spirit IDs and catch_time are raw server values, not positional indexes.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", star_tower_query);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "settle_floor_fight",
        star_tower_settle_floor_fight,
        storey_index: i64,
        node_index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "get_floor_award",
        star_tower_get_floor_award,
        storey_index: i64
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "quick_fight",
        star_tower_quick_fight,
        storey: i64,
        storey1: i64,
        sell: bool
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "toggle_auto_sell",
        star_tower_toggle_auto_sell
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "settle_top_boss_fight",
        star_tower_settle_top_boss_fight
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "get_top_reward",
        star_tower_get_top_reward,
        reward_index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "query_bag", star_tower_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "full_level",
        star_tower_full_level,
        spirit_id: i64,
        catch_time: i64
    );
}
