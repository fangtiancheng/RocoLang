use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

// Index convention:
// - week_task::claim_task index is 0-based, matching AS UI cell index.
// - week_task::exchange exchange_type: 0=pet reward, 1=exchange item; index is passed through to CGI.
// - diamond_task::claim_reward index is 1-based, matching the CGI value sent by AS.
pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(
        module,
        stdlib,
        "week_task_query",
        play_guide_week_task_query
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "week_task_claim_task",
        play_guide_week_task_claim_task,
        index: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "week_task_exchange",
        play_guide_week_task_exchange,
        exchange_type: i64,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "diamond_task_query",
        play_guide_diamond_task_query
    );
    register_stdlib_fn_1!(
        module,
        stdlib,
        "diamond_task_claim_reward",
        play_guide_diamond_task_claim_reward,
        index: i64
    );
    register_stdlib_fn_0!(
        module,
        stdlib,
        "qq_game_hall_gift",
        play_guide_qq_game_hall_gift
    );
}
