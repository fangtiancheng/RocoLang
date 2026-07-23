use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    lock_stdlib, parse_i64_array, register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2,
    to_array, to_rhai_error,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_ground_info", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_get_ground_info().map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_seed_bag", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_get_seed_bag()
                .map(|items| to_array(&items))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_plant_status", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_get_plant_status()
                .map(|statuses| to_array(&statuses))
                .map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_1!(module, stdlib, "reclaim", manor_reclaim, ground_id: i64);
    register_stdlib_fn_2!(module, stdlib, "sow", manor_sow, seed_id: i64, ground_id: i64);
    register_stdlib_fn_1!(module, stdlib, "reap", manor_reap, ground_id: i64);
    register_stdlib_fn_1!(module, stdlib, "uproot", manor_uproot, ground_id: i64);
    register_stdlib_fn_2!(module, stdlib, "weed", manor_weed, ground_id: i64, weed_type: i64);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "use_fertilizer",
        manor_use_fertilizer,
        ground_id: i64,
        fertilizer_item_id: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "submit_strawman_result",
        manor_submit_strawman_result,
        game_result: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "claim_strawman_reward",
        manor_claim_strawman_reward
    );
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("claim_strawman_gift", move || {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_claim_strawman_gift()
                .map(|rewards| to_array(&rewards))
                .map_err(to_rhai_error)
        });
    }
    register_stdlib_fn_0!(module, stdlib, "query_coco_tree", manor_query_coco_tree);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "apply_coco_tree_feed",
        manor_apply_coco_tree_feed,
        feed_type: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "query_friend_coco_tree",
        manor_query_friend_coco_tree,
        friend_uin: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "apply_friend_coco_tree_feed",
        manor_apply_friend_coco_tree_feed,
        friend_uin: i64
    );
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_friend_list", move |version: i64| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_get_friend_list(version)
                .map(|friends| to_array(&friends))
                .map_err(to_rhai_error)
        });
    }
    {
        let stdlib = stdlib.clone();
        module.set_native_fn("get_friend_details", move |friend_uins: rhai::Array| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.manor_get_friend_details(parse_i64_array("friend_uins[]", friend_uins)?)
                .map(|friends| to_array(&friends))
                .map_err(to_rhai_error)
        });
    }
}
