use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - first_settle_fight prop_index is 0-based, matching ui1468 AS id: 0..3.
// - second_settle_fight npc_index is 0-based, matching ui1469 AS id: 0..4.
// - third_exchange_item exchange_position is 1-based, matching ui1471 excPos: 1=light, 2=tail.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", libra_first_query);
    register_stdlib_fn_0!(module, stdlib, "first_submit_game", libra_first_submit_game);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "first_settle_fight",
        libra_first_settle_fight,
        prop_index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_get_gift", libra_first_get_gift);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_notify_full_level",
        libra_first_notify_full_level,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_query_bag", libra_first_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_advance",
        libra_first_advance,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(module, stdlib, "second_query", libra_second_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_settle_fight",
        libra_second_settle_fight,
        npc_index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_awaken", libra_second_awaken);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_buy_challenge_count",
        libra_second_buy_challenge_count
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_full_level",
        libra_second_full_level,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_query_bag", libra_second_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_evolution",
        libra_second_evolution,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_query_status",
        libra_third_query_status
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_exchange_item",
        libra_third_exchange_item,
        exchange_position: i64
    );
    register_stdlib_fn_1!(module, stdlib, "third_buy_tail", libra_third_buy_tail, count: i64);
    register_stdlib_fn_0!(module, stdlib, "third_buy_wish", libra_third_buy_wish);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_exchange_pet",
        libra_third_exchange_pet
    );
}
