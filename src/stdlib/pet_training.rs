use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::{register_stdlib_fn_1, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_1!(
        module,
        stdlib,
        "query",
        pet_training_query,
        training_type: i64
    );
    register_stdlib_fn_2!(
        module,
        stdlib,
        "settle",
        pet_training_settle,
        training_type: i64,
        pet_id: i64
    );
}
