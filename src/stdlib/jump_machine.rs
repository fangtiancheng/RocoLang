use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::register_stdlib_fn_0;
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", jump_machine_query);
    register_stdlib_fn_0!(module, stdlib, "play", jump_machine_play);
}
