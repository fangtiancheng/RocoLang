use std::sync::{Arc, Mutex};

use rhai::Module;

use crate::stdlib::util::register_stdlib_fn_0;
use crate::stdlib::RocoStdLib;

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query", diamond_tear_query);
    register_stdlib_fn_0!(module, stdlib, "buy_pet", diamond_tear_buy_pet);
    register_stdlib_fn_0!(module, stdlib, "freeze", diamond_tear_freeze);
    register_stdlib_fn_0!(module, stdlib, "claim_diamond", diamond_tear_claim_diamond);
}
