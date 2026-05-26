use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_vanguard_lord(module, stdlib.clone());
    register_little_angel_angie(module, stdlib.clone());
    register_red_lotus_beast(module, stdlib.clone());
    register_blue_water_beast(module, stdlib.clone());
    register_ice_crystal_tiger(module, stdlib.clone());
    register_dark_pioneer_dragon(module, stdlib.clone());
    register_bat_prince(module, stdlib.clone());
    register_rock_armor_lord(module, stdlib.clone());
    register_golden_mantis(module, stdlib.clone());
    register_loki(module, stdlib.clone());
    register_magic_tail_cat(module, stdlib.clone());
    register_nether_fox(module, stdlib.clone());
    register_drill_man(module, stdlib);
}

fn register_vanguard_lord<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_query",
        magic_pioneer_vanguard_lord_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_submit_combat",
        magic_pioneer_vanguard_lord_submit_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_put",
        magic_pioneer_vanguard_lord_put
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_buy",
        magic_pioneer_vanguard_lord_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_get_reward",
        magic_pioneer_vanguard_lord_get_reward
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "vanguard_lord_exchange",
        magic_pioneer_vanguard_lord_exchange,
        index: i64,
        num: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "vanguard_lord_complete",
        magic_pioneer_vanguard_lord_complete
    );
}

fn register_little_angel_angie<T: RocoStdLib + 'static>(
    module: &mut Module,
    stdlib: Arc<Mutex<T>>,
) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "little_angel_angie_query",
        magic_pioneer_little_angel_angie_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "little_angel_angie_buy",
        magic_pioneer_little_angel_angie_buy
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "little_angel_angie_submit_combat",
        magic_pioneer_little_angel_angie_submit_combat,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "little_angel_angie_get_reward",
        magic_pioneer_little_angel_angie_get_reward
    );
}

fn register_red_lotus_beast<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_query",
        magic_pioneer_red_lotus_beast_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_buy",
        magic_pioneer_red_lotus_beast_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_submit",
        magic_pioneer_red_lotus_beast_submit
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "red_lotus_beast_start_fight",
        magic_pioneer_red_lotus_beast_start_fight,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_settle_fight",
        magic_pioneer_red_lotus_beast_settle_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "red_lotus_beast_get_gift",
        magic_pioneer_red_lotus_beast_get_gift
    );
}

fn register_blue_water_beast<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_query",
        magic_pioneer_blue_water_beast_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_one_key",
        magic_pioneer_blue_water_beast_one_key
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_check",
        magic_pioneer_blue_water_beast_check
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_submit_game",
        magic_pioneer_blue_water_beast_submit_game
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "blue_water_beast_submit_combat",
        magic_pioneer_blue_water_beast_submit_combat,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "blue_water_beast_get_gift",
        magic_pioneer_blue_water_beast_get_gift
    );
}

fn register_ice_crystal_tiger<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_query",
        magic_pioneer_ice_crystal_tiger_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_one_key",
        magic_pioneer_ice_crystal_tiger_one_key
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_submit_game",
        magic_pioneer_ice_crystal_tiger_submit_game
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "ice_crystal_tiger_submit_chain",
        magic_pioneer_ice_crystal_tiger_submit_chain,
        index: i64,
        number: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "ice_crystal_tiger_start_indexed_fight",
        magic_pioneer_ice_crystal_tiger_start_indexed_fight,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_report_fight",
        magic_pioneer_ice_crystal_tiger_report_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "ice_crystal_tiger_get_gift",
        magic_pioneer_ice_crystal_tiger_get_gift
    );
}

fn register_dark_pioneer_dragon<T: RocoStdLib + 'static>(
    module: &mut Module,
    stdlib: Arc<Mutex<T>>,
) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_query",
        magic_pioneer_dark_pioneer_dragon_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_buy",
        magic_pioneer_dark_pioneer_dragon_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_submit_combat",
        magic_pioneer_dark_pioneer_dragon_submit_combat
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_add",
        magic_pioneer_dark_pioneer_dragon_add
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "dark_pioneer_dragon_get_reward",
        magic_pioneer_dark_pioneer_dragon_get_reward
    );
}

fn register_bat_prince<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bat_prince_query",
        magic_pioneer_bat_prince_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bat_prince_one_key",
        magic_pioneer_bat_prince_one_key
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "bat_prince_start_indexed_fight",
        magic_pioneer_bat_prince_start_indexed_fight,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bat_prince_submit",
        magic_pioneer_bat_prince_submit
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "bat_prince_get_gift",
        magic_pioneer_bat_prince_get_gift
    );
}

fn register_rock_armor_lord<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_query",
        magic_pioneer_rock_armor_lord_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_buy",
        magic_pioneer_rock_armor_lord_buy
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "rock_armor_lord_submit_combat",
        magic_pioneer_rock_armor_lord_submit_combat,
        index: i64,
        success: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_add",
        magic_pioneer_rock_armor_lord_add
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "rock_armor_lord_get_reward",
        magic_pioneer_rock_armor_lord_get_reward
    );
}

fn register_golden_mantis<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_query",
        magic_pioneer_golden_mantis_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_buy",
        magic_pioneer_golden_mantis_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_start_fight",
        magic_pioneer_golden_mantis_start_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_settle_fight",
        magic_pioneer_golden_mantis_settle_fight
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "golden_mantis_submit",
        magic_pioneer_golden_mantis_submit,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "golden_mantis_get_gift",
        magic_pioneer_golden_mantis_get_gift
    );
}

fn register_loki<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "loki_query", magic_pioneer_loki_query);
    register_stdlib_fn_0!(module, stdlib, "loki_buy", magic_pioneer_loki_buy);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "loki_submit_combat",
        magic_pioneer_loki_submit_combat
    );
    register_stdlib_fn_0!(module, stdlib, "loki_add", magic_pioneer_loki_add);
    register_stdlib_fn_0!(module, stdlib, "loki_get_gift", magic_pioneer_loki_get_gift);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "loki_query_all",
        magic_pioneer_loki_query_all
    );
}

fn register_magic_tail_cat<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_query",
        magic_pioneer_magic_tail_cat_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_buy",
        magic_pioneer_magic_tail_cat_buy
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_start_fight",
        magic_pioneer_magic_tail_cat_start_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_settle_fight",
        magic_pioneer_magic_tail_cat_settle_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_learn",
        magic_pioneer_magic_tail_cat_learn
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_submit",
        magic_pioneer_magic_tail_cat_submit
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "magic_tail_cat_get_gift",
        magic_pioneer_magic_tail_cat_get_gift
    );
}

fn register_nether_fox<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "nether_fox_query",
        magic_pioneer_nether_fox_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "nether_fox_buy",
        magic_pioneer_nether_fox_buy
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "nether_fox_start_fight",
        magic_pioneer_nether_fox_start_fight,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "nether_fox_settle_fight",
        magic_pioneer_nether_fox_settle_fight
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "nether_fox_submit",
        magic_pioneer_nether_fox_submit,
        number: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "nether_fox_get_gift",
        magic_pioneer_nether_fox_get_gift
    );
}

fn register_drill_man<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_query",
        magic_pioneer_drill_man_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_one_key",
        magic_pioneer_drill_man_one_key
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_start_fight",
        magic_pioneer_drill_man_start_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_report_fight",
        magic_pioneer_drill_man_report_fight
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "drill_man_submit",
        magic_pioneer_drill_man_submit,
        index: i64,
        success: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "drill_man_get_gift",
        magic_pioneer_drill_man_get_gift
    );
}
