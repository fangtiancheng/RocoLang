use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", four_seasons_query);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "roll_dice",
        four_seasons_roll_dice,
        vip: bool
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "submit_minigame",
        four_seasons_submit_minigame
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "upgrade_box",
        four_seasons_upgrade_box,
        position: i64
    );
    // tool=1/2 maps to AS double/slow items. The appointed-step item requires
    // a selected step and is exposed as use_appointed_step_tool(step).
    register_stdlib_fn_1!(module, stdlib, "use_tool", four_seasons_use_tool, tool: i64);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "use_appointed_step_tool",
        four_seasons_use_appointed_step_tool,
        step: i64
    );
    register_stdlib_fn_0!(module, stdlib, "submit_event", four_seasons_submit_event);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "exchange_shop_item",
        four_seasons_exchange_shop_item,
        slot: i64
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "claim_spirit_reward",
        four_seasons_claim_spirit_reward,
        index: i64
    );
}
