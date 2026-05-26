use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2, register_stdlib_fn_3,
};
use crate::stdlib::RocoStdLib;

// Index convention:
// - second kind: 0=direct, 1=mini-game, 2=boss fight.
// - third side: 0=sun, 1=moon.
// - third index: 0-based, matching ui1279 sun/moon arrays.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", gemini_first_query);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_upgrade",
        gemini_first_upgrade,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_upgrade_to_100",
        gemini_first_upgrade_to_100,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "first_buy_ingredient",
        gemini_first_buy_ingredient
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "first_buy_evolve_access",
        gemini_first_buy_evolve_access,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "first_query_bag", gemini_first_query_bag);

    register_stdlib_fn_0!(module, stdlib, "second_query", gemini_second_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_submit",
        gemini_second_submit,
        kind: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_get_gift", gemini_second_get_gift);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "second_add_score",
        gemini_second_add_score,
        kind: i64,
        score: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_buy", gemini_second_buy);

    register_stdlib_fn_0!(module, stdlib, "third_query", gemini_third_query);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_settle_combat",
        gemini_third_settle_combat,
        side: i64,
        index: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_submit",
        gemini_third_submit,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_submit_without_spirit",
        gemini_third_submit_without_spirit
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_buy_level",
        gemini_third_buy_level,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "third_buy_score", gemini_third_buy_score);
    register_stdlib_fn_0!(module, stdlib, "third_query_bag", gemini_third_query_bag);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_buy_challenge_count",
        gemini_third_buy_challenge_count
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "third_buy_score_by_index",
        gemini_third_buy_score_by_index,
        side: i64,
        index: i64,
        score: i64
    );
}
