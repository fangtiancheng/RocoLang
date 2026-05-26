use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - second_settle reward_source is the raw AS id: 0=first NPC, 1=second NPC, 2=mine.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", scorpio_first_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "first_submit_game",
        scorpio_first_submit_game,
        score: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_query_bag", scorpio_first_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_evolve",
        scorpio_first_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_level_up",
        scorpio_first_level_up,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_buy_direct", scorpio_first_buy_direct);

    register_stdlib_fn_0!(module, stdlib, "second_query", scorpio_second_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_settle",
        scorpio_second_settle,
        reward_source: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_query_bag", scorpio_second_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_evolve",
        scorpio_second_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_level_up",
        scorpio_second_level_up,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_buy_direct",
        scorpio_second_buy_direct
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_buy_challenge_count",
        scorpio_second_buy_challenge_count
    );

    register_stdlib_fn_0!(module, stdlib, "third_query", scorpio_third_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_submit_game",
        scorpio_third_submit_game,
        success: bool
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_exchange_pet",
        scorpio_third_exchange_pet
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_buy_guess_count",
        scorpio_third_buy_guess_count,
        count: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_buy_red_sand",
        scorpio_third_buy_red_sand,
        count: i64
    );
}
