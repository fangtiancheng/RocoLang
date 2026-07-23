use std::sync::{Arc, Mutex};

use rhai::{Array, Module, NativeCallContext};

use crate::stdlib::util::{
    lock_stdlib, parse_i64_array, register_stdlib_fn_0, register_stdlib_fn_1,
    to_rhai_error_in_context,
};
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
    let stdlib = stdlib.clone();
    module.set_native_fn(
        "fuse",
        move |context: NativeCallContext,
              recipe_index: i64,
              material_bag_indexes: Array,
              personality: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.mystery_fusion_fuse(
                recipe_index,
                parse_i64_array("material_bag_indexes[]", material_bag_indexes)?,
                personality,
            )
            .map_err(|error| to_rhai_error_in_context(error, &context))
        },
    );
}
