use std::sync::{Arc, Mutex};

use rhai::{Engine, Module};

use super::*;

macro_rules! register_stdlib_module {
    ($engine:expr, $stdlib:expr, $module:ident) => {{
        let mut rhai_module = Module::new();
        $module::register(&mut rhai_module, $stdlib.clone());
        $engine.register_static_module(stringify!($module), rhai_module.into());
    }};
}

macro_rules! register_value_module {
    ($modules:expr, $module:ident) => {{
        let mut rhai_module = Module::new();
        $module::register(&mut rhai_module);
        $modules.push((stringify!($module), rhai_module));
    }};
}

pub(crate) fn registered_value_modules() -> Vec<(&'static str, Module)> {
    let mut modules = Vec::new();
    register_value_module!(modules, spirit_book_state);
    register_value_module!(modules, evolution_edge_kind);
    register_value_module!(modules, personality);
    register_value_module!(modules, weather);
    register_value_module!(modules, combat_status);
    register_value_module!(modules, combat_result);
    modules
}

pub(crate) fn register_modules<T: RocoStdLib + 'static>(
    engine: &mut Engine,
    stdlib: Arc<Mutex<T>>,
) {
    register_stdlib_module!(engine, stdlib, scene);
    register_stdlib_module!(engine, stdlib, remote_state);
    register_stdlib_module!(engine, stdlib, session);
    register_stdlib_module!(engine, stdlib, memory);
    register_stdlib_module!(engine, stdlib, profile);
    register_stdlib_module!(engine, stdlib, role);
    register_stdlib_module!(engine, stdlib, game);
    register_stdlib_module!(engine, stdlib, ladder);
    register_stdlib_module!(engine, stdlib, type_ladder);
    register_stdlib_module!(engine, stdlib, spirit);
    register_stdlib_module!(engine, stdlib, spirit_book);
    for (name, module) in registered_value_modules() {
        engine.register_static_module(name, module.into());
    }
    register_stdlib_module!(engine, stdlib, home);
    register_stdlib_module!(engine, stdlib, manor);
    register_stdlib_module!(engine, stdlib, pet_training);
    register_stdlib_module!(engine, stdlib, news);
    register_stdlib_module!(engine, stdlib, news_times);
    register_stdlib_module!(engine, stdlib, task);
    register_stdlib_module!(engine, stdlib, incubative_machine);
    register_stdlib_module!(engine, stdlib, pet_egg);
    register_stdlib_module!(engine, stdlib, star_tower);
    register_stdlib_module!(engine, stdlib, sentinel_intelligence);
    register_stdlib_module!(engine, stdlib, mountain_sea);
    register_stdlib_module!(engine, stdlib, magic_pioneer);
    register_stdlib_module!(engine, stdlib, alchemy_furnace);
    register_stdlib_module!(engine, stdlib, adventure);
    register_stdlib_module!(engine, stdlib, unicorn);
    register_stdlib_module!(engine, stdlib, four_seasons);
    register_stdlib_module!(engine, stdlib, ice_crystal);
    register_stdlib_module!(engine, stdlib, multi_evolution);
    register_stdlib_module!(engine, stdlib, diamond_tear);
    register_stdlib_module!(engine, stdlib, dark_city);
    register_stdlib_module!(engine, stdlib, mystery_fusion);
    register_stdlib_module!(engine, stdlib, treasure_realm);
    register_stdlib_module!(engine, stdlib, summon);
    register_stdlib_module!(engine, stdlib, play_guide);
    register_stdlib_module!(engine, stdlib, jump_machine);
    register_stdlib_module!(engine, stdlib, capricorn);
    register_stdlib_module!(engine, stdlib, cancer);
    register_stdlib_module!(engine, stdlib, virgo);
    register_stdlib_module!(engine, stdlib, pisces);
    register_stdlib_module!(engine, stdlib, taurus);
    register_stdlib_module!(engine, stdlib, three_starters);
    register_stdlib_module!(engine, stdlib, gemini);
    register_stdlib_module!(engine, stdlib, sagittarius);
    register_stdlib_module!(engine, stdlib, aquarius);
    register_stdlib_module!(engine, stdlib, aries);
    register_stdlib_module!(engine, stdlib, scorpio);
    register_stdlib_module!(engine, stdlib, libra);
    register_stdlib_module!(engine, stdlib, leo);
    register_stdlib_module!(engine, stdlib, lookup);
    register_stdlib_module!(engine, stdlib, combat);
    register_stdlib_module!(engine, stdlib, system);

    let mut global_system_module = Module::new();
    system::register_functions(&mut global_system_module, stdlib);
    engine.register_global_module(global_system_module.into());
}
