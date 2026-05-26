use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2, register_stdlib_fn_3,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_cultivation_query",
        alchemy_furnace_monkey_cultivation_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_cultivation_submit_default",
        alchemy_furnace_monkey_cultivation_submit_default
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "monkey_cultivation_submit_pills",
        alchemy_furnace_monkey_cultivation_submit_pills,
        dragon_tiger: bool,
        cultivate_origin: bool,
        nine_turn: bool
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_cultivation_get_gift",
        alchemy_furnace_monkey_cultivation_get_gift
    );

    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_evo_query",
        alchemy_furnace_monkey_evo_query
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "monkey_evo_report_fight",
        alchemy_furnace_monkey_evo_report_fight,
        fight_type: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_evo_give_up",
        alchemy_furnace_monkey_evo_give_up
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_evo_submit_default",
        alchemy_furnace_monkey_evo_submit_default
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "monkey_evo_submit_pills",
        alchemy_furnace_monkey_evo_submit_pills,
        dragon_tiger: bool,
        cultivate_origin: bool,
        nine_turn: bool
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_evo_query_bag",
        alchemy_furnace_monkey_evo_query_bag
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "monkey_evo_evolve",
        alchemy_furnace_monkey_evo_evolve,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "monkey_evo_get_gift",
        alchemy_furnace_monkey_evo_get_gift
    );

    register_stdlib_fn_0!(
        module,
        stdlib,
        "raging_fire_query",
        alchemy_furnace_raging_fire_query
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "raging_fire_submit_stone",
        alchemy_furnace_raging_fire_submit_stone,
        count: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "raging_fire_report_fight",
        alchemy_furnace_raging_fire_report_fight,
        target: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "raging_fire_buy",
        alchemy_furnace_raging_fire_buy,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "raging_fire_query_bag",
        alchemy_furnace_raging_fire_query_bag
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "raging_fire_get_gift",
        alchemy_furnace_raging_fire_get_gift,
        spirit_id: i64,
        catch_time: i64
    );
}
