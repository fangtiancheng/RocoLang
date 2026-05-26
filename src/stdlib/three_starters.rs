use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1};
use crate::stdlib::RocoStdLib;

// Index convention:
// - fire index is 1-based, matching the CGI index sent by the original AS.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "water_query", three_starters_water_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "water_buy",
        three_starters_water_buy,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "water_settle_fight",
        three_starters_water_settle_fight
    );
    register_stdlib_fn_0!(module, stdlib, "water_submit", three_starters_water_submit);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "water_get_gift",
        three_starters_water_get_gift,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "water_query_bag",
        three_starters_water_query_bag
    );

    register_stdlib_fn_0!(module, stdlib, "fire_query", three_starters_fire_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_buy",
        three_starters_fire_buy,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_start",
        three_starters_fire_start,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_submit_battle",
        three_starters_fire_submit_battle,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_submit_direct",
        three_starters_fire_submit_direct,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_over",
        three_starters_fire_over,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_get_gift",
        three_starters_fire_get_gift,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "fire_query_bag",
        three_starters_fire_query_bag
    );

    register_stdlib_fn_0!(module, stdlib, "sun_query", three_starters_sun_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "sun_buy",
        three_starters_sun_buy,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sun_start_fight",
        three_starters_sun_start_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sun_settle_fight",
        three_starters_sun_settle_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sun_start_collect",
        three_starters_sun_start_collect
    );
    register_stdlib_fn_0!(module, stdlib, "sun_submit", three_starters_sun_submit);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "sun_get_gift",
        three_starters_sun_get_gift,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sun_query_bag",
        three_starters_sun_query_bag
    );
}
