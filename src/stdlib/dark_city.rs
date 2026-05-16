use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - reputation exchange index is 0-based, matching repu_get CGI and the
//   DarkCityExchangeItem.index returned by reputation_query_exchange().
// - expedition_start_fight only prepares the NPC combat and returns fight_id;
//   scripts should start the combat separately with combat::start_combat.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "expedition_query",
        dark_city_expedition_query
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "expedition_start_fight",
        dark_city_expedition_start_fight,
        vip_boost: bool
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "expedition_settle_fight",
        dark_city_expedition_settle_fight
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "expedition_set_vip_pass",
        dark_city_expedition_set_vip_pass,
        enabled: bool
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "reputation_query_exchange",
        dark_city_reputation_query_exchange
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "reputation_exchange",
        dark_city_reputation_exchange,
        index: i64,
        count: i64
    );
}
