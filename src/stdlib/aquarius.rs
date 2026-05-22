use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - first_settle_combat boss_index is 0-based, matching ui1879 btnCombat_0..5.
// - second_exchange_item exchange_position is 1-based, matching ui1890 excPos: 1=light, 2=tail.
// - third_query_bag bag_type follows CGI directly; AS sends type=1 for evolve candidates.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "first_query", aquarius_first_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "first_settle_combat",
        aquarius_first_settle_combat,
        boss_index: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_buy_evolve_access",
        aquarius_first_buy_evolve_access,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "first_add_challenge_count",
        aquarius_first_add_challenge_count
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "first_buy_star_num",
        aquarius_first_buy_star_num
    );
    register_stdlib_fn_0!(module, stdlib, "first_query_bag", aquarius_first_query_bag);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_evolve",
        aquarius_first_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "first_buy_direct",
        aquarius_first_buy_direct,
        item_or_spirit_id: i64,
        count: i64
    );

    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_query_status",
        aquarius_second_query_status
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_exchange_item",
        aquarius_second_exchange_item,
        exchange_position: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_query_diamond",
        aquarius_second_query_diamond
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_combat_again",
        aquarius_second_combat_again
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "second_buy_tail",
        aquarius_second_buy_tail,
        count: i64
    );
    register_stdlib_fn_0!(module, stdlib, "second_buy_wish", aquarius_second_buy_wish);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_exchange_pet",
        aquarius_second_exchange_pet
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "second_buy_spirit",
        aquarius_second_buy_spirit
    );

    register_stdlib_fn_0!(module, stdlib, "third_query", aquarius_third_query);
    register_stdlib_fn_0!(module, stdlib, "third_random", aquarius_third_random);
    register_stdlib_fn_0!(
        module,
        stdlib,
        "third_settle_combat",
        aquarius_third_settle_combat
    );
    register_stdlib_fn_0!(module, stdlib, "third_buy_level", aquarius_third_buy_level);
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_buy_evolve",
        aquarius_third_buy_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "third_evolve",
        aquarius_third_evolve,
        spirit_id: i64,
        catch_time: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "third_query_bag",
        aquarius_third_query_bag,
        bag_type: i64
    );
}
