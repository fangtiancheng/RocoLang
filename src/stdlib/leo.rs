use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - first_exchange_item exchange_position is 1-based, matching ui1350 excPos: 1=light, 2=tail.
// - second_settle_combat hunt_index is 1-based, matching ui1351 submit index: 1..=6.
// - third_submit_combat challenge_index is 0-based, matching ui1357 challenge index: 0..=4.
// - third_light_star star_index is 0-based, matching ui1357 star index: 0..=4.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query_status", leo_first_query_status);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "first_exchange_item",
        leo_first_exchange_item,
        exchange_position: i64
    );
    register_stdlib_fn_1!(module, stdlib, "first_buy_tail", leo_first_buy_tail, count: i64);
    register_stdlib_fn_0!(module, stdlib, "first_buy_wish", leo_first_buy_wish);
    register_stdlib_fn_0!(module, stdlib, "first_exchange_pet", leo_first_exchange_pet);

    register_stdlib_fn_0!(module, stdlib, "second_query", leo_second_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_settle_combat",
        leo_second_settle_combat,
        hunt_index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_submit_onekey",
        leo_second_submit_onekey
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_query_spirit",
        leo_second_query_spirit
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_submit_spirit",
        leo_second_submit_spirit,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_buy_full_level",
        leo_second_buy_full_level,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(module, stdlib, "third_query", leo_third_query);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_submit_combat",
        leo_third_submit_combat,
        challenge_index: i64,
        win: bool
    );
    register_stdlib_fn_0!(module, stdlib, "third_get_reward", leo_third_get_reward);
    register_stdlib_fn_0!(module, stdlib, "third_full_level", leo_third_full_level);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_evolve",
        leo_third_evolve,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_light_star",
        leo_third_light_star,
        star_index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "third_query_bag", leo_third_query_bag);
}
