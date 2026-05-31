use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{
    lock_stdlib, register_stdlib_fn_0, register_stdlib_fn_1, register_stdlib_fn_3, to_rhai_error,
};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "get_pause", get_pause);
    register_stdlib_fn_1!(module, stdlib, "set_pause", set_pause, enabled: bool);
    register_stdlib_fn_1!(
        module,
        stdlib,
        "start_mini_game",
        start_mini_game,
        game_id: i64
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "submit_mini_game_score",
        submit_mini_game_score,
        game_id: i64,
        score: i64,
        game_type: i64
    );
    register_stdlib_fn_3!(
        module,
        stdlib,
        "try_submit_mini_game_score",
        try_submit_mini_game_score,
        game_id: i64,
        score: i64,
        game_type: i64
    );

    {
        let stdlib = stdlib.clone();
        module.set_native_fn("try_set_pause", move |enabled: bool| {
            let mut lib = lock_stdlib(&stdlib)?;
            lib.try_set_pause(enabled).map_err(to_rhai_error)
        });
    }
}
