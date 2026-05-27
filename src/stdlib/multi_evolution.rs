use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_3, register_stdlib_fn_4,
    register_stdlib_fn_5,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_1!(
        module,
        stdlib,
        "fire_query_candidates",
        multi_evolution_fire_query_candidates,
        slot: i64
    );
    register_stdlib_fn_5!(
        module,
        stdlib,
        "fire_evolve",
        multi_evolution_fire_evolve,
        slot: i64,
        spirit_id: i64,
        catch_time: i64,
        item_count: i64,
        fire_score: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "fire_query_booster_item_count",
        multi_evolution_fire_query_booster_item_count
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "fire_claim_reward",
        multi_evolution_fire_claim_reward
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "fire_query_reward_available",
        multi_evolution_fire_query_reward_available
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "water_query_candidates",
        multi_evolution_water_query_candidates,
        slot: i64
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "water_evolve",
        multi_evolution_water_evolve,
        slot: i64,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "grass_query_candidates",
        multi_evolution_grass_query_candidates,
        slot: i64
    );
    register_stdlib_fn_4!(
        module,
        stdlib,
        "grass_first_evolve",
        multi_evolution_grass_first_evolve,
        slot: i64,
        spirit_id: i64,
        catch_time: i64,
        sunlight: i64
    );
    register_stdlib_fn_4!(
        module,
        stdlib,
        "grass_second_evolve",
        multi_evolution_grass_second_evolve,
        slot: i64,
        spirit_id: i64,
        catch_time: i64,
        sunlight: i64
    );
}
