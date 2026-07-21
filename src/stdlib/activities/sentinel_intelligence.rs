use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - boss_index and exchange/evolve index are 1-based, matching the original AS
//   activity UI and sentinel_intelligence_new CGI parameters.
// - Returned Sentinel*Info.index fields use the same 1-based convention, so
//   scripts can pass them back directly.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", sentinel_intelligence_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "start_fight",
        sentinel_intelligence_start_fight,
        boss_index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "settle_fight",
        sentinel_intelligence_settle_fight
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "refresh_mission",
        sentinel_intelligence_refresh_mission
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "refresh_boss",
        sentinel_intelligence_refresh_boss
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "refresh_exchange",
        sentinel_intelligence_refresh_exchange
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "exchange_item",
        sentinel_intelligence_exchange_item,
        index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "exchange_spirit",
        sentinel_intelligence_exchange_spirit,
        index: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "evolve_spirit",
        sentinel_intelligence_evolve_spirit,
        index: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(module, stdlib, "query_all", sentinel_intelligence_query_all);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "get_prize",
        sentinel_intelligence_get_prize,
        boss_index: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "query_bag",
        sentinel_intelligence_query_bag,
        evolve_spirit_id: i64
    );
}
