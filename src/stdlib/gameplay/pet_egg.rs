use crate::stdlib::util::{register_stdlib_fn_0, register_stdlib_fn_2};
use crate::stdlib::RocoStdLib;
use rhai::Module;
use std::sync::{Arc, Mutex};

pub fn register<T: RocoStdLib + 'static>(module: &mut Module, stdlib: Arc<Mutex<T>>) {
    register_stdlib_fn_0!(module, stdlib, "query_info", pet_egg_query_info);
    register_stdlib_fn_0!(module, stdlib, "vip_speed_up", pet_egg_vip_speed_up);
    register_stdlib_fn_2!(module, stdlib, "begin", pet_egg_begin, male_index: i64, female_index: i64);
    register_stdlib_fn_0!(module, stdlib, "cancel", pet_egg_cancel);
    register_stdlib_fn_2!(module, stdlib, "preview", pet_egg_preview, male_index: i64, female_index: i64);
}
