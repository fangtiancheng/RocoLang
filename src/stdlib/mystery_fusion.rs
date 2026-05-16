use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_3};
use crate::stdlib::RocoStdLib;

// Index convention:
// - battle_index and recipe_index are 1-based, matching the CGI and values
//   returned by query().
// - material_bag_indexes are PetInfo.index values returned by query_material_bag().
// - personality <= 0 means no inherited personality; positive values are sent as CGI type.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", mystery_fusion_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "prepare_battle",
        mystery_fusion_prepare_battle,
        battle_index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_battle",
        mystery_fusion_submit_battle
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "query_material_bag",
        mystery_fusion_query_material_bag,
        spirit_id: i64
    );
    register_stdlib_fn_0!(module, stdlib, "claim_reward", mystery_fusion_claim_reward);
    register_stdlib_fn_3!(
        module,
        stdlib,
        "fuse",
        mystery_fusion_fuse,
        recipe_index: i64,
        material_bag_indexes: Vec<i64>,
        personality: i64
    );
}
