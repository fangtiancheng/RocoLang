use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sharp_scorpion_query",
        cancer_sharp_scorpion_query
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "sharp_scorpion_exchange_item",
        cancer_sharp_scorpion_exchange_item,
        exc_pos: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "sharp_scorpion_buy_tail",
        cancer_sharp_scorpion_buy_tail,
        num: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sharp_scorpion_buy_wish",
        cancer_sharp_scorpion_buy_wish
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "sharp_scorpion_exc_pet",
        cancer_sharp_scorpion_exc_pet
    );
    register_stdlib_fn_0!(module, stdlib, "mend_shape_query", cancer_mend_shape_query);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "mend_shape_upgrade",
        cancer_mend_shape_upgrade,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "mend_shape_upgrade_to_100",
        cancer_mend_shape_upgrade_to_100,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "mend_shape_buy",
        cancer_mend_shape_buy,
        buy_type: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "mend_shape_buy_full",
        cancer_mend_shape_buy_full,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "mend_shape_query_bag",
        cancer_mend_shape_query_bag
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "unseal_memories_query",
        cancer_unseal_memories_query
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "unseal_memories_start_game",
        cancer_unseal_memories_start_game
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "unseal_memories_commit",
        cancer_unseal_memories_commit,
        choice: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "unseal_memories_bag_query",
        cancer_unseal_memories_bag_query
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "unseal_memories_put_full",
        cancer_unseal_memories_put_full,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "unseal_memories_buy",
        cancer_unseal_memories_buy,
        buy_type: i64
    );
}
