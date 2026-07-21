use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", unicorn_query);
    register_stdlib_fn_0!(module, stdlib, "summon", unicorn_summon);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "settle_summon_fight",
        unicorn_settle_summon_fight,
        slot: i64
    );
    register_stdlib_fn_0!(module, stdlib, "harvest", unicorn_harvest);
    register_stdlib_fn_0!(module, stdlib, "submit_minigame", unicorn_submit_minigame);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "start_cultivation",
        unicorn_start_cultivation
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_cultivation_task",
        unicorn_submit_cultivation_task,
        task: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "claim_cultivation_reward",
        unicorn_claim_cultivation_reward
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "query_evolution_bag",
        unicorn_query_evolution_bag,
        stage: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "evolve",
        unicorn_evolve,
        stage: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "one_key_evolve",
        unicorn_one_key_evolve,
        stage: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "exchange_skill_stone",
        unicorn_exchange_skill_stone,
        skill: i64,
        cost_kind: i64
    );
}
