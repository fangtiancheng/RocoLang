use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_3};
use crate::stdlib::RocoStdLib;

// Index convention:
// - boss_index is 1-based, matching the mystery_uncharted CGI `index`
//   parameter and MountainSeaBossInfo.index returned by query().
// - page_index and soul_type for summon are also 1-based, matching the CGI
//   `index` and `type` parameters. soul_type = 0 means no soul boost.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", mountain_sea_query);
    register_stdlib_fn_0!(module, stdlib, "open", mountain_sea_open);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "enter_boss",
        mountain_sea_enter_boss,
        boss_index: i64
    );
    register_stdlib_fn_0!(module, stdlib, "settle_fight", mountain_sea_settle_fight);
    register_stdlib_fn_3!(
        module,
        stdlib,
        "summon",
        mountain_sea_summon,
        page_index: i64,
        soul_type: i64,
        soul_count: i64
    );
}
