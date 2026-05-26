use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - first_settle_battle battle_type follows AS CGI directly: 1=sky soldier, 2=progress boss.
// - third_exchange_item exchange_position is 1-based, matching ui1213 excPos: 1=light, 2=tail.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", aries_first_query);
    register_stdlib_fn_0!(module, stdlib, "first_start", aries_first_start);
    register_stdlib_fn_0!(module, stdlib, "first_buy_times", aries_first_buy_times);
    register_stdlib_fn_0!(module, stdlib, "first_dice", aries_first_dice);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "first_settle_battle",
        aries_first_settle_battle,
        battle_type: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_query_bag", aries_first_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_level_up",
        aries_first_level_up,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_evolve",
        aries_first_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_buy_direct", aries_first_buy_direct);
    register_stdlib_fn_0!(module, stdlib, "first_get_gold", aries_first_get_gold);

    register_stdlib_fn_0!(module, stdlib, "second_query", aries_second_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_submit_game",
        aries_second_submit_game,
        power: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_query_bag", aries_second_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_evolve",
        aries_second_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_buy_direct", aries_second_buy_direct);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_buy_level",
        aries_second_buy_level,
        spirit_id: i64,
        catch_time: i64
    );

    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_query_status",
        aries_third_query_status
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_exchange_item",
        aries_third_exchange_item,
        exchange_position: i64
    );
    register_stdlib_fn_1!(module, stdlib, "third_buy_tail", aries_third_buy_tail, count: i64);
    register_stdlib_fn_0!(module, stdlib, "third_buy_wish", aries_third_buy_wish);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_exchange_pet",
        aries_third_exchange_pet
    );
}
