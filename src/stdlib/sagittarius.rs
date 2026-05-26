use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - first_complete index: 1-based CGI star-picture index.
// - second task_type: raw AS type, 0=accept/finish, 1=give up.
// - second bag_type: raw AS type, 0=level candidates, 1=evolve/buy candidates.
// - third boss/score index: 0-based, matching ui1836 arrays.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", sagittarius_first_query);
    register_stdlib_fn_0!(module, stdlib, "first_submit", sagittarius_first_submit);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "first_query_scene",
        sagittarius_first_query_scene
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "first_pick_shine",
        sagittarius_first_pick_shine
    );
    register_stdlib_fn_0!(module, stdlib, "first_add_map", sagittarius_first_add_map);
    register_stdlib_fn_0!(module, stdlib, "first_show_map", sagittarius_first_show_map);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "first_add_speed",
        sagittarius_first_add_speed
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "first_complete",
        sagittarius_first_complete,
        index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_get_pet", sagittarius_first_get_pet);

    register_stdlib_fn_0!(module, stdlib, "second_query", sagittarius_second_query);
    register_stdlib_fn_0!(module, stdlib, "second_start", sagittarius_second_start);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_level_up",
        sagittarius_second_level_up,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_buy_up",
        sagittarius_second_buy_up,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_random", sagittarius_second_random);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_task",
        sagittarius_second_task,
        task_type: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_query_bag",
        sagittarius_second_query_bag,
        bag_type: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_spirit_up",
        sagittarius_second_spirit_up,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(module, stdlib, "third_query", sagittarius_third_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_settle_combat",
        sagittarius_third_settle_combat,
        boss_index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "third_buy_pet", sagittarius_third_buy_pet);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_buy_evolve",
        sagittarius_third_buy_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_add_challenge_count",
        sagittarius_third_add_challenge_count
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_buy_star_num",
        sagittarius_third_buy_star_num,
        score_index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_query_bag",
        sagittarius_third_query_bag
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_up",
        sagittarius_third_up,
        spirit_id: i64,
        catch_time: i64
    );
}
